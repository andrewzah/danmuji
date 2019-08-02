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
    models::reply::{NewReply,Reply},
    utils,
    BotData,
};

group!({
    name: "replies",
    options: {
        prefixes: ["r", "replies"],
    },
    commands: [list, set, delete]
});

#[command]
fn set(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let tag: String = args.single()?;
    let url: String = args.single()?;

    let guild_id = msg.guild_id.ok_or("Replies don't work in direct messages.")?;
    info!("gid: {}", &guild_id);

    let reply = NewReply {
        guild_id: &guild_id.to_string(),
        tag: &tag,
        url: &url
    };

    info!("new: {:?}", &reply);

    match db::upsert_reply(&reply) {
        Ok(_) => {
            let message = format!("Sucessfully set tag {}.", tag);
            let _ = msg.channel_id.say(&ctx, &message);
            Ok(())
        },
        Err(e) => Err(CommandError::from(e))
    }
}

#[command]
fn list(ctx: &mut Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.ok_or("Replies don't work in direct messages.")?;

    match db::get_replies(&guild_id.to_string()) {
        Ok(reply_list) => {
            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.title("Replies");
                        e.description(reply_list.pretty_print());

                        e
                    });

                    m
            });
            Ok(())
        },
        Err(e) => Err(CommandError::from(e))
    }
}

#[command]
#[aliases("del")]
fn delete(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = msg.guild_id.ok_or("Replies don't work in direct messages.")?;
    let tag: String = args.single()?;

    match db::delete_reply(&tag, &guild_id.to_string()) {
        Ok(result) => {
            match result {
                0 => msg.reply(&ctx, "Nothing to delete."),
                _ => msg.reply(&ctx, "Deleted reply.")
            };
            Ok(())
        },
        Err(e) => Err(CommandError::from(e))
    }
}
