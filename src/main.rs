#![allow(dead_code)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use std::{
    collections::HashSet,
    env,
    sync::Arc,
    time::{Duration, Instant},
};

use diesel_migrations::embed_migrations;
use dotenv::dotenv;
use hey_listen::sync::ParallelDispatcher as Dispatcher;
use log::{error, info};
use serenity::{
    client::bridge::gateway::ShardManager,
    framework::standard::{DispatchError, StandardFramework},
    model::id::UserId,
    prelude::*,
};
use white_rabbit::Scheduler;

mod checks;
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
    replies::REPLIES_GROUP,
};
use dispatch::{DispatchEvent, DispatcherKey, SchedulerKey};
use errors::AppError;
use handler::Handler;
use models::message::NewMessage;

struct ShardManagerContainer;
impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct BotData {
    pub start_time: Instant,
    pub last_insertion: Instant,
    pub message_queue: Vec<NewMessage>,
    pub disabled_channel_ids: Vec<u64>,
}
impl TypeMapKey for BotData {
    type Value = Arc<Mutex<BotData>>;
}
impl BotData {
    pub fn insert_messages(&mut self) {
        let thirty_secs = Duration::from_secs(30);
        let elapsed = self.last_insertion.elapsed();
        let len = self.message_queue.len();

        if len >= 100 || elapsed > thirty_secs {
            match db::insert_messages(&self.message_queue) {
                Ok(count) => {
                    info!("Inserted {} messages.", count);
                    self.last_insertion = Instant::now();
                    self.message_queue = vec![];
                },
                Err(err) => error!("err inserting messages: {}", err),
            }
        }
    }
}

fn migrate() {
    // run before to fully migrate
    let conn = db::pool()
        .clone()
        .get()
        .expect("Unable to get db connection on migrate startup!!");
    let _ = embedded_migrations::run_with_output(&conn, &mut std::io::stdout());
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

embed_migrations!("./migrations");

fn main() {
    dotenv().ok();
    init_logging();
    migrate();

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
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        data.insert::<db::DbPool>(db::pool());
        data.insert::<DispatcherKey>(Arc::new(RwLock::new(dispatcher)));
        data.insert::<SchedulerKey>(scheduler);

        let bot_data = BotData {
            start_time: Instant::now(),
            last_insertion: Instant::now(),
            message_queue: vec![],
            disabled_channel_ids: db::disabled_channel_ids()
                .expect("Unable to load disabled channels!"),
        };
        data.insert::<BotData>(Arc::new(Mutex::new(bot_data)));
    }

    let _bot_id = get_bot_id(&client);
    let bot_prefix = match env::var("DANMUJI_PREFIX") {
        Ok(prefix) => prefix,
        Err(_) => String::from("yi "),
    };

    let (owners, bot_id) = match client.cache_and_http.http.get_current_application_info() {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    client.with_framework(
        StandardFramework::new()
            .bucket("leaderboard", |b| b.delay(1).time_span(1440).limit(1))
            .configure(|c| {
                c.prefix(&bot_prefix)
                    .on_mention(Some(bot_id))
                    .delimiters(vec![", ", ","])
                    .owners(owners)
            })
            .on_dispatch_error(|ctx, msg, error| {
                if let DispatchError::Ratelimited(seconds) = error {
                    let _ = msg.channel_id.say(
                        &ctx.http,
                        &format!("Try this again in {} seconds.", seconds),
                    );
                }
            })
            .after(|ctx, msg, _cmd_name, res| {
                let _ = res.map_err(|err| {
                    AppError::from(err).send_err(&ctx.http, msg, "Unable to run command".into())
                });
            })
            .help(&HELP)
            .group(&GENERAL_GROUP)
            .group(&CHANNELS_GROUP)
            .group(&HANGEUL_GROUP)
            .group(&REPLIES_GROUP),
    );

    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}
