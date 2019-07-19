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
    models::{channel::NewChannel, user::NewUser},
    utils,
    BotData,
};

group!({
    name: "replies",
    options: {
        prefixes: ["r"],
    },
    commands: [ ]
});

// --------------------------------------------------------------------
// ---------------------------- helpers -------------------------------
// --------------------------------------------------------------------
