#[macro_use]
extern crate diesel;

use std::{collections::{HashMap, HashSet}, env, fmt::Write, sync::Arc};

use dotenv::dotenv;
use log::{info, debug, error};
use serenity::{
    client::bridge::gateway::{ShardId, ShardManager},
    framework::standard::{
        Args, CheckResult, CommandOptions, CommandResult, CommandGroup,
        DispatchError, HelpOptions, help_commands, StandardFramework,
        macros::{command, group, help, check},
    },
    model::{channel::{Channel, Message}, gateway::Ready, id::{UserId, GuildId}, event::ResumedEvent},
    utils::{content_safe, ContentSafeOptions},
};
use serenity::prelude::*;

mod db;
mod errors;
mod schema;
mod utils;

const BOT_ID: u64 = 592184706896756736;

group!({
    name: "general",
    options: {},
    commands: [ping],
});

struct ShardManagerContainer;
impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }

    fn message(&self, mut ctx: Context, msg: Message) {
        if msg.author.id == BOT_ID { return }

        let formatted_content = utils::format_content(&msg.content);
        let m = db::NewMessage {
            message_id: &msg.id.to_string(),
            guild_id: &msg.guild_id.unwrap_or(GuildId(0_u64)).to_string(),
            channel_id: &msg.channel_id.to_string(),
            user_id: &msg.author.id.to_string(),
            hangeul_count: 0,
            non_hangeul_count: 1,
            raw_count: 2,
            time: msg.timestamp,
        };

        //parse_content(&formatted_content);
        match db::insert_message(&mut ctx, m) {
            Ok(u) => info!("finished inserting msg: usize? {}", u),
            Err(err) => error!(":x: error: {}", err)
        }
    }

    fn resume(&self, _: Context, resume: ResumedEvent) {
        debug!("Resumed; trace: {:?}", resume.trace);
    }
}

fn parse_content(content: &str) {
    let mut non_hangeul = 0;
    let mut hangeul = 0;

    let blocks = content.trim().split("");
    for block in blocks {
        for character in block.chars() {
            if utils::is_hangeul(character) {
                hangeul += 1;
            } else {
                non_hangeul += 1;
            }
        }
    }
}

#[help]
fn my_help(
    context: &mut Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>
) -> CommandResult {
    help_commands::with_embeds(context, msg, args, help_options, groups, owners)
}

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;

    Ok(())
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

    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("yi "))
        .group(&GENERAL_GROUP));

    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

