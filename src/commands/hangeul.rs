use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        CommandError,
        CommandResult,
    },
    model::channel::Message,
    utils::Colour,
};

use crate::{checks::*, commands::{channels::*, roles::*}, db, models::user::NewUser, utils};


group!({
    name: "hangeul",
    options: {
        prefixes: ["hangeul", "hangul"],
    },
    commands: [
        opt_in, opt_out, leaderboard,
        reset_guild, reset_all
    ],
    sub_groups: [
        ROLES,
        CHANNELS
    ]
});

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
#[bucket = "leaderboard"]
#[only_in(guilds)]
fn leaderboard(ctx: &mut Context, msg: &Message) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => return Err(CommandError("".into()))
    };

    let _ = &msg.channel_id.broadcast_typing(&ctx.http);

    match db::get_leaderboard(&guild_id.to_string()) {
        Ok(list) => {
            let text = list.pretty_print(&ctx.http)?;

                utils::send_message(&msg.channel_id, &ctx, |m| {
                    m.embed(|e| {
                        e.title("한글/English Leaderboard");
                        e.description(text);
                        e.colour(Colour::DARK_GOLD);

                        e
                    });
                    m
                })
        },
        Err(err) => Err(CommandError::from(err)),
    }
}

#[command]
#[allowed_roles("Admin", "Administrator")]
#[only_in(guilds)]
fn reset_guild(ctx: &mut Context, msg: &Message) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => return Err(CommandError("".into()))
    };

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
