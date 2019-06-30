#[macro_use]
extern crate diesel;

use std::{env, sync::Arc};

use dotenv::dotenv;
use serenity::prelude::*;
use serenity::{client::bridge::gateway::ShardManager, framework::standard::StandardFramework};

mod commands;
mod db;
mod errors;
mod handlers;
mod schema;
mod utils;

use commands::groups::*;
use handlers::events::Handler;

struct ShardManagerContainer;
impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

fn init_logging() {
    let mut builder = env_logger::Builder::from_default_env();
    builder.target(env_logger::Target::Stdout);
    builder.init()
}

fn main() {
    dotenv().ok();
    init_logging();

    let discord_token = &env::var("DISCORD_TOKEN").expect("Is DISCORD_TOKEN set?");

    let mut client = Client::new(discord_token, Handler).expect("Error creating client");
    {
        let mut data = client.data.write();

        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        data.insert::<db::DbConn>(db::connection());
    }

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("yi "))
            .group(&GENERAL_GROUP),
    );

    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}
