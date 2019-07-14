use std::collections::HashSet;

use log::{debug, info};
use serenity::{
    client::Context,
    framework::standard::{
        help_commands,
        macros::{command, help},
        Args,
        CommandError,
        CommandGroup,
        CommandResult,
        HelpOptions,
    },
    model::{channel::Message, id::UserId},
};

use crate::{utils, BotData};

#[command]
fn uptime(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();
    let bot_data = data.get::<BotData>().expect("Expected BotData");
    let elapsed = bot_data.read().start_time.elapsed().as_secs();

    let resp = format!("Uptime: {}", utils::format_seconds(elapsed));

    let _ = msg.channel_id.say(&ctx.http, resp);
    Ok(())
}
