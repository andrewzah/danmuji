use serenity::framework::standard::macros::group;

use crate::commands::{general::*, reminders::*};

group!({
    name: "general",
    options: {},
    commands: [ping],
});

group!({
    name: "remind_me",
    options: {},
    commands: [add_reminder],
});
