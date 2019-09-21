use std::error::Error;

use log::error;
use serenity::{
    builder::CreateMessage,
    framework::standard::{CommandError, CommandResult},
    model::prelude::*,
    prelude::*,
};

pub fn reply(msg: &Message, ctx: &Context, text: &str) -> CommandResult {
    match msg.reply(ctx, text) {
        Err(err) => {
            error!("Unable to reply: {}", err.description());
            Err(CommandError(err.description().into()))
        },
        _ => Ok(()),
    }
}

pub fn say(channel_id: &ChannelId, ctx: &Context, msg: &str) -> CommandResult {
    match channel_id.say(ctx, msg) {
        Err(err) => {
            error!("Unable to say: {}", err.description());
            Err(CommandError(err.description().into()))
        },
        _ => Ok(()),
    }
}

pub fn send_message<'a, F>(channel_id: &ChannelId, ctx: &Context, f: F) -> CommandResult
where
    for<'b> F: FnOnce(&'b mut CreateMessage<'a>) -> &'b mut CreateMessage<'a>,
{
    match channel_id.send_message(&ctx.http, f) {
        Err(err) => {
            error!("Unable to send message: {}", err.description());
            Err(CommandError(err.description().into()))
        },
        _ => Ok(()),
    }
}

