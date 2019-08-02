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
    model::{channel::Message, id::UserId},
};

use crate::{
    db,
    models::{channel::NewChannel, user::NewUser},
    BotData,
};

group!({
    name: "hangeul",
    options: {
        prefixes: ["hangeul", "hangul", "h"],
    },
    commands: [
        opt_in, opt_out, ratio_results
    ],
});

#[command]
fn ratio(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;
    Ok(())
}

fn change_user_opt_out(opt_out: bool, user_id: String, msg: &Message) -> CommandResult {
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
    let result = change_user_opt_out(true, msg.author.id.to_string(), msg);
    match result {
        Ok(_) => {
            msg.channel_id.say(
                &ctx.http,
                format!("<@:{}> has successfully opted out.", msg.author.id),
            );
            Ok(())
        },
        Err(err) => Err(err),
    }
}

#[command]
fn opt_in(ctx: &mut Context, msg: &Message) -> CommandResult {
    let result = change_user_opt_out(false, msg.author.id.to_string(), msg);
    match result {
        Ok(_) => {
            msg.channel_id.say(
                &ctx.http,
                format!("<@:{}> has successfully opted in.", msg.author.id),
            );
            Ok(())
        },
        Err(err) => Err(err),
    }
}

#[command]
fn ratio_results(ctx: &mut Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.ok_or("Replies don't work in direct messages.")?;
    let _ = &msg.channel_id.broadcast_typing(&ctx.http);

    match db::get_ratio_list(&guild_id.to_string()) {
        Ok(list) => {
            msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title("한글/English Ratio Results");
                    e.description(list.pretty_print(&ctx.http));

                    e
                });
                m
            });
            Ok(())
        },
        Err(err) => Err(CommandError::from(err)),
    }
}
