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
        prefixes: ["r"],
    },
    commands: [list, set]
});

#[command]
fn set(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let tag: String = args.single()?;
    let url: String = args.single()?;
    let reply = NewReply { tag: &tag, url: &url };

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
    match db::get_replies() {
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
