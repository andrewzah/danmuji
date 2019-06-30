use serenity::framework::standard::macros::group;

#[allow(dead_code)]
use crate::commands::{
    general::*,
    reminders::*,
    //queries::*,
};

group!({
    name: "general",
    options: {},
    commands: [ping],
});

group!({
    name: "remind_me",
    options: {
        prefixes: ["rm", "reminder"],
    },
    commands: [set_reminder],
});
