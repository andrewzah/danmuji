use std::collections::HashSet;

use log::{info, debug};
use serenity::{
    client::Context,
    framework::standard::{
        help_commands,
        macros::{command, help},
        Args,
        CommandGroup,
        CommandResult,
        CommandError,
        HelpOptions,
    },
    model::{channel::Message, id::UserId},
};

use crate::{
    db,
    commands::groups::*,
    models::user::NewUser};

#[help]
fn my_help(
    context: &mut Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    help_commands::with_embeds(context, msg, args, help_options, groups, owners)
}

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;
    Ok(())
}

#[command]
fn ratio(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;
    Ok(())
}

fn change_user_opt_out(opt_out: bool, user_id: String, msg: &Message) -> CommandResult {
    let new_user = NewUser {
        opt_out: opt_out,
        user_id: &msg.author.id.to_string()
    };

    match db::upsert_user(&new_user) {
        Ok(_) => { Ok(()) },
        Err(err) => Err(CommandError::from(err))
    }
}

#[command]
fn opt_out(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let result = change_user_opt_out(true, msg.author.id.to_string(), msg);
    match result {
        Ok(_) => {
            msg.channel_id.say(&ctx.http, format!("<@:{}> has successfully opted out.", msg.author.id));
            Ok(())
        },
        Err(err) => Err(err)
    }
}

#[command]
fn opt_in(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let result = change_user_opt_out(false, msg.author.id.to_string(), msg);
    match result {
        Ok(_) => {
            msg.channel_id.say(&ctx.http, format!("<@:{}> has successfully opted in.", msg.author.id));
            Ok(())
        },
        Err(err) => Err(err)
    }
}

#[command]
fn ratio_results(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = &msg.channel_id.broadcast_typing(&ctx.http);

    match db::get_ratio_list() {
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
        Err(err) => Err(CommandError::from(err))
    }
}
