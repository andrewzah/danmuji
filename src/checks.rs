use log::info;
use serenity::{
    framework::standard::{macros::check, Args, CheckResult, CommandOptions},
    model::channel::Message,
    prelude::*,
};

#[check]
#[name = "Owner"]
#[display_in_help(true)]
fn owner_check(_: &mut Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> CheckResult {
    //(msg.author.id == 7).into()
    if msg.author.id == 91329651909074944_u64 {
        info!("success");
        CheckResult::Success
    } else {
        info!("failure");
        CheckResult::new_user("This command is restricted to andrei#7237.")
    }
}

#[check]
#[name = "Admin"]
#[display_in_help(true)]
fn admin_check(ctx: &mut Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> CheckResult {
    if let Some(member) = msg.member(&ctx.cache) {
        if let Ok(permissions) = member.permissions(&ctx.cache) {
            return permissions.administrator().into();
        }
    }

    false.into()
}
