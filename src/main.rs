#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

#[macro_use]
extern crate diesel;

use std::{env, sync::Arc};

use dotenv::dotenv;
use hey_listen::sync::ParallelDispatcher as Dispatcher;
use log::error;
use serenity::{
    client::bridge::gateway::ShardManager,
    framework::standard::{DispatchError, StandardFramework},
    model::id::UserId,
    prelude::*,
};
use white_rabbit::Scheduler;

mod commands;
mod db;
mod dispatch;
mod errors;
mod handler;
mod models;
mod schema;
mod tasks;
mod utils;

use commands::{
    channels::CHANNELS_GROUP,
    general::{GENERAL_GROUP, HELP},
    hangeul::HANGEUL_GROUP,
    reminders::REMIND_ME_GROUP,
};
use dispatch::{DispatchEvent, DispatcherKey, SchedulerKey};
use errors::{AppError, ErrorKind};
use handler::Handler;
use models::channel::ChannelId;

struct ShardManagerContainer;
impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct BotData {
    pub disabled_channel_ids: Vec<u64>,
    pub start_time: std::time::Instant,
}
impl TypeMapKey for BotData {
    type Value = Arc<RwLock<BotData>>;
}

fn init_logging() {
    let mut builder = env_logger::Builder::from_default_env();
    builder.target(env_logger::Target::Stdout);
    builder.init()
}

fn get_bot_id(client: &Client) -> UserId {
    match client.cache_and_http.http.get_current_application_info() {
        Ok(info) => return info.id,
        Err(why) => panic!("Could not access application info: {:?}", why),
    }
}

fn main() {
    dotenv().ok();
    init_logging();

    let discord_token = &env::var("DISCORD_TOKEN").expect("Is DISCORD_TOKEN set?");

    let scheduler = Scheduler::new(4);
    let scheduler = Arc::new(RwLock::new(scheduler));

    let mut dispatcher: Dispatcher<DispatchEvent> = Dispatcher::default();
    dispatcher
        .num_threads(4)
        .expect("Could not construct dispatcher threadpool");

    let mut client = Client::new(discord_token, Handler).expect("Error creating client");
    {
        let mut data = client.data.write();
        let bot_data = BotData {
            disabled_channel_ids: db::disabled_channel_ids()
                .expect("Unable to load disabled channels!"),
            start_time: std::time::Instant::now(),
        };

        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        data.insert::<db::DbPool>(db::pool());
        data.insert::<DispatcherKey>(Arc::new(RwLock::new(dispatcher)));
        data.insert::<SchedulerKey>(scheduler);
        data.insert::<BotData>(Arc::new(RwLock::new(bot_data)));
    }

    let bot_id = get_bot_id(&client);

    client.with_framework(
        StandardFramework::new()
            .configure(|c| {
                c.prefix("yi ")
                    .on_mention(Some(bot_id))
                    .delimiters(vec![", ", ","])
            })
            .on_dispatch_error(|ctx, msg, error| {
                if let DispatchError::Ratelimited(seconds) = error {
                    let _ = msg.channel_id.say(
                        &ctx.http,
                        &format!("Try this again in {} seconds.", seconds),
                    );
                }
            })
            .after(|ctx, msg, cmd_name, res| {
                res.map_err(|err| {
                    AppError::from(err).send_err(&ctx.http, msg, "Unable to run command".into())
                });
            })
            .help(&HELP)
            .group(&GENERAL_GROUP)
            .group(&CHANNELS_GROUP)
            .group(&HANGEUL_GROUP)
            .group(&REMIND_ME_GROUP),
    );

    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}
