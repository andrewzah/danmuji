#[macro_use]
extern crate diesel;

use std::{env, sync::Arc};

use dotenv::dotenv;
use serenity::{client::bridge::gateway::ShardManager, framework::standard::StandardFramework, model::id::UserId};
use serenity::prelude::*;
use hey_listen::sync::{ParallelDispatcher as Dispatcher,
ParallelDispatcherRequest as DispatcherRequest};
use white_rabbit::Scheduler;

mod commands;
mod db;
mod dispatch;
mod errors;
mod handler;
mod schema;
mod utils;

use commands::groups::*;
use dispatch::{DispatchEvent, DispatcherKey, SchedulerKey};
use handler::Handler;

struct ShardManagerContainer;
impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

fn init_logging() {
    let mut builder = env_logger::Builder::from_default_env();
    builder.target(env_logger::Target::Stdout);
    builder.init()
}

fn get_bot_id(client: Client) -> UserId {
    match client.cache_and_http.http.get_current_application_info() {
        Ok(info) => {
            return info.id
        },
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
    dispatcher.num_threads(4).expect("Could not construct threadpool");

    let mut client = Client::new(discord_token, Handler).expect("Error creating client");
    {
        let mut data = client.data.write();

        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        data.insert::<db::DbConn>(db::connection());
        data.insert::<DispatcherKey>(Arc::new(RwLock::new(dispatcher)));
        data.insert::<SchedulerKey>(scheduler);
    }

    let bot_id = get_bot_id(client);

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c
                .prefix("yi ")
                .on_mention(Some(bot_id))
            )
            .group(&GENERAL_GROUP),
    );

    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}
