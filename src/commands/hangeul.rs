use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        CommandError,
        CommandResult,
    },
    model::channel::Message,
};

use crate::{checks::*, db, models::user::NewUser, utils};

group!({
    name: "hangeul",
    options: {
        prefixes: ["hangeul", "hangul", "h"],
    },
    commands: [
        opt_in, opt_out, ratio_results,
        reset_guild, reset_all
    ],
});

#[command]
fn ratio(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;
    Ok(())
}

fn change_user_opt_out(opt_out: bool, msg: &Message) -> CommandResult {
    let new_user = NewUser {
        opt_out: opt_out,
        user_id: &msg.author.id.to_string(),
    };

    match db::upsert_user(&new_user) {
        Ok(_) => Ok(()),
        Err(err) => Err(CommandError::from(err)),
    }
}

#[command]
fn opt_out(ctx: &mut Context, msg: &Message) -> CommandResult {
    let result = change_user_opt_out(true, msg);
    match result {
        Ok(_) => utils::reply(&msg, &ctx, "Successfully opted out"),
        Err(err) => Err(err),
    }
}

#[command]
fn opt_in(ctx: &mut Context, msg: &Message) -> CommandResult {
    let result = change_user_opt_out(false, msg);
    match result {
        Ok(_) => utils::say(
            &msg.channel_id,
            &ctx,
            &format!("<@:{}> has successfully opted in.", msg.author.id),
        ),
        Err(err) => Err(err),
    }
}

#[command]
fn ratio_results(ctx: &mut Context, msg: &Message) -> CommandResult {
    let guild_id = msg
        .guild_id
        .ok_or("Replies don't work in direct messages.")?;
    let _ = &msg.channel_id.broadcast_typing(&ctx.http);

    match db::get_ratio_list(&guild_id.to_string()) {
        Ok(list) => utils::send_message(&msg.channel_id, &ctx, |m| {
            m.embed(|e| {
                e.title("한글/English Ratio Results");
                e.description(list.pretty_print(&ctx.http));

                e
            });
            m
        }),
        Err(err) => Err(CommandError::from(err)),
    }
}

#[command]
#[checks(Admin)]
fn reset_guild(ctx: &mut Context, msg: &Message) -> CommandResult {
    let guild_id = msg
        .guild_id
        .ok_or("Replies don't work in direct messages.")?;

    match db::delete_guild_messages(&guild_id.to_string()) {
        Ok(count) => utils::say(
            &msg.channel_id,
            &ctx,
            &format!("Cleared {} messages from this guild.", count),
        ),
        Err(err) => Err(CommandError::from(err)),
    }
}

#[command]
#[checks(Owner)]
fn reset_all(ctx: &mut Context, msg: &Message) -> CommandResult {
    match db::delete_all_messages() {
        Ok(count) => utils::say(
            &msg.channel_id,
            &ctx,
            &format!("Cleared {} messages from all guilds.", count),
        ),
        Err(err) => Err(CommandError::from(err)),
    }
}
