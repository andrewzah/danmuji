use log::{debug, info};
use serenity::{
    client::Context,
    framework::standard::{
        help_commands,
        macros::{command, group, help},
        Args,
        CommandError,
        CommandGroup,
        CommandResult,
        HelpOptions,
    },
    model::{
        channel::{GuildChannel, Message},
        id::{ChannelId, UserId},
    },
};

use crate::{
    db,
    errors::{AppError, ErrorKind, Result},
    models::reply::{NewReply,Reply},
    utils,
    BotData,
};

group!({
    name: "replies",
    options: {
        prefixes: ["r"],
    },
    commands: [get, set]
});

#[command]
fn set(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let tag: String = args.single()?;
    let url: String = args.single()?;
    let reply = NewReply { tag: &tag, url: &url };

    match db::upsert_reply(&reply) {
        Ok(_) => {
            let message = format!("Sucessfully set tag {}.", tag);
            let _ = msg.channel_id.say(&ctx, &message);
            Ok(())
        },
        Err(e) => Err(CommandError::from(e))
    }
}

#[command]
fn get(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let tag: String = args.single()?;

    match db::get_reply(&tag) {
        Ok(reply) => {
            let message = format!("Found tag `{}`: {}", tag, reply.url);
            let _ = msg.reply(&ctx, &message);
            Ok(())
        },
        Err(e) => Err(CommandError::from(e))
    }
}
