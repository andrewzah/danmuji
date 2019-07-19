use std::collections::HashSet;

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
    model::{channel::Message, id::UserId},
};

use crate::{utils, BotData};

group!({
    name: "general",
    options: {},
    commands: [uptime],
});

#[help]
fn help(
    context: &mut Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    help_commands::with_embeds(context, msg, args, help_options, groups, owners)
}

#[command]
fn uptime(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();
    let bot_data = data.get::<BotData>().expect("Expected BotData");
    let elapsed = bot_data.read().start_time.elapsed().as_secs();

    let resp = format!("Uptime: {}", utils::format_seconds(elapsed));

    let _ = msg.channel_id.say(&ctx.http, resp);
    Ok(())
}
