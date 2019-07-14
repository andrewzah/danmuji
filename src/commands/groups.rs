use std::collections::HashSet;

use serenity::{
    client::Context,
    framework::standard::{
        help_commands,
        macros::{group, help},
        Args,
        CommandError,
        CommandGroup,
        CommandResult,
        HelpOptions,
    },
    model::{channel::Message, id::UserId},
};

use crate::commands::{channels::*, general::*, hangeul::*, reminders::*};

group!({
    name: "general",
    options: {},
    commands: [uptime],
});

group!({
    name: "hangeul",
    options: {
        prefixes: ["hangeul", "hangul", "h"],
    },
    commands: [
        opt_in, opt_out, ratio_results
    ],
});

group!({
    name: "remind_me",
    options: {},
    commands: [add_reminder],
});

group!({
    name: "channels",
    options: {
        prefixes: ["chan", "c"],
    },
    commands: [
        list, enable, disable,
        disable_all, enable_all,
    ],
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
