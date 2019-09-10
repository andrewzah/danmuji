use log::info;
use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        Args,
        CommandError,
        CommandResult,
    },
    model::channel::Message,
};

use crate::{db, models::reply::NewReply, utils};

group!({
    name: "replies",
    options: {
        allowed_roles: [
            "Mod", "Moderator", "Admin", "Administrator",
        ],
        prefixes: ["r", "replies"],
    },
    commands: [list, set, delete]
});

#[command]
fn set(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let tag: String = args.single()?;
    let url: String = args.single()?;

    let guild_id = msg
        .guild_id
        .ok_or("Replies don't work in direct messages.")?;
    info!("gid: {}", &guild_id);

    let reply = NewReply {
        guild_id: &guild_id.to_string(),
        tag: &tag,
        url: &url,
    };

    info!("new: {:?}", &reply);

    match db::upsert_reply(&reply) {
        Ok(_) => {
            let message = format!("Sucessfully set tag {}.", tag);
            let _ = msg.channel_id.say(&ctx, &message);
            Ok(())
        },
        Err(e) => Err(CommandError::from(e)),
    }
}

#[command]
fn list(ctx: &mut Context, msg: &Message) -> CommandResult {
    let guild_id = msg
        .guild_id
        .ok_or("Replies don't work in direct messages.")?;

    match db::get_replies(&guild_id.to_string()) {
        Ok(reply_list) => utils::send_message(&msg.channel_id, &ctx, |m| {
            m.embed(|e| {
                e.title("Replies");
                e.description(reply_list.pretty_print());

                e
            });

            m
        }),
        Err(e) => Err(CommandError::from(e)),
    }
}

#[command]
#[aliases("del")]
fn delete(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = msg
        .guild_id
        .ok_or("Replies don't work in direct messages.")?;
    let tag: String = args.single()?;

    match db::delete_reply(&tag, &guild_id.to_string()) {
        Ok(result) => {
            match result {
                0 => utils::say(&msg.channel_id, &ctx, "Nothing to delete.")?,
                _ => utils::say(&msg.channel_id, &ctx, "Deleted reply.")?,
            };
            Ok(())
        },
        Err(e) => Err(CommandError::from(e)),
    }
}
