use serenity::{
    framework::standard::{macros::check, Args, CheckResult, CommandOptions},
    model::channel::Message,
    prelude::*,
};

use crate::utils;

#[check]
#[name = "Owner"]
#[check_in_help(true)]
#[display_in_help(false)]
fn owner_check(ctx: &mut Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> CheckResult {
    if msg.author.id == 91329651909074944_u64 {
        true.into()
    } else {
        let _ = utils::say(&msg.channel_id, &ctx, "Only <@91329651909074944> can run this command. :eyes:");
        false.into()
    }
}

#[check]
#[name = "Admin"]
#[check_in_help(true)]
fn admin_check(ctx: &mut Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> CheckResult {
    if let Some(member) = msg.member(&ctx.cache) {
        if let Ok(permissions) = member.permissions(&ctx.cache) {
            return permissions.administrator().into();
        }
    }

    false.into()
}
