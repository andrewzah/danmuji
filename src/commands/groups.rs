use serenity::framework::standard::macros::group;

use crate::commands::{general::*, reminders::*};

group!({
    name: "general",
    options: {},
    commands: [ping],
});

group!({
    name: "hangeul",
    options: {
        prefixes: ["hangeul", "hangul", "h"],
    },
    commands: [opt_in, opt_out, ratio_results],
});

group!({
    name: "remind_me",
    options: {},
    commands: [add_reminder],
});
