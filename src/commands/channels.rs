use log::info;
use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        Args,
        CommandError,
        CommandResult,
    },
    model::channel::{GuildChannel, Message},
    utils::Colour,
};

use crate::{
    db,
    errors::{AppError, Result},
    models::channel::NewChannel,
    utils,
    BotData,
};


/// Lists all enabled or disabled channels for a guild.
#[command]
pub fn list(ctx: &mut Context, msg: &Message) -> CommandResult {
    let channel_list = match db::enabled_channels() {
        Ok(list) => list,
        Err(err) => return Err(CommandError::from(err)),
    };

    let description = match channel_list.pretty_print(&msg, &ctx.http) {
        Ok(desc) => desc,
        Err(err) => return Err(CommandError::from(err))
    };

    utils::send_message(&msg.channel_id, &ctx, |m| {
        m.embed(|e| {
            e.colour(Colour::DARK_GOLD);
            e.title("Enabled Channels");
            e.description(description);

            e
        });

        m
    })
}

#[command]
pub fn enable(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    match args.len() {
        0 => {
            let new_channel = vec![NewChannel {
                channel_id: msg.channel_id.to_string(),
                enabled: true,
            }];
            update_channels(new_channel, true, ctx, msg)
        },
        _ => {
            let chan_ids = utils::format_channels(args.single()?)?;
            let new_channels = chan_ids
                .into_iter()
                .map(|cid| NewChannel {
                    channel_id: cid,
                    enabled: true,
                })
                .collect();

            update_channels(new_channels, true, ctx, msg)
        },
    }
}

#[command]
fn disable(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    match args.len() {
        0 => {
            let new_channel = vec![NewChannel {
                channel_id: msg.channel_id.to_string(),
                enabled: false,
            }];
            update_channels(new_channel, false, ctx, msg)
        },
        _ => {
            let chan_ids = utils::format_channels(args.single()?)?;
            let new_channels = chan_ids
                .into_iter()
                .map(|cid| NewChannel {
                    channel_id: cid,
                    enabled: false,
                })
                .collect();

            update_channels(new_channels, false, ctx, msg)
        },
    }
}

#[command]
pub fn enable_all(ctx: &mut Context, msg: &Message) -> CommandResult {
    let channels = all_guild_channels(ctx, msg)?;
    let new_channels = channels
        .into_iter()
        .map(|cid| NewChannel {
            channel_id: cid.id.to_string(),
            enabled: true,
        })
        .collect();

    update_channels(new_channels, true, ctx, msg)
}

#[command]
pub fn disable_all(ctx: &mut Context, msg: &Message) -> CommandResult {
    let channels = all_guild_channels(ctx, msg)?;
    let new_channels = channels
        .into_iter()
        .map(|cid| NewChannel {
            channel_id: cid.id.to_string(),
            enabled: false,
        })
        .collect();

    update_channels(new_channels, false, ctx, msg)
}

// --------------------------------------------------------------------
// ---------------------------- helpers -------------------------------
// --------------------------------------------------------------------

fn refresh_disabled_channel_ids(ctx: &Context) -> CommandResult {
    let data_lock = ctx
        .data
        .read()
        .get::<BotData>()
        .cloned()
        .expect("Expected BotData");
    let mut bot_data = data_lock.lock();

    match db::disabled_channel_ids() {
        Ok(ids) => {
            bot_data.disabled_channel_ids = ids;
            Ok(())
        },
        Err(err) => Err(CommandError::from(err)),
    }
}

fn all_guild_channels(ctx: &Context, msg: &Message) -> Result<Vec<GuildChannel>> {
    let guild_id = match msg.guild_id {
        Some(gid) => gid.as_u64().clone(),
        None => return Err(AppError::from_str("TODO")),
    };

    ctx.http
        .get_channels(guild_id)
        .map_err(|e| AppError::from(e))
}

fn update_channels(
    channels: Vec<NewChannel>,
    enabled: bool,
    ctx: &mut Context,
    msg: &Message,
) -> CommandResult {
    let message = match channels.len() {
        0 => String::from("No (valid) channels specified. Example: `yi c enable #announcements`."),
        _ => {
            let verb = match enabled {
                true => "Enabled",
                false => "Disabled",
            };

            let plural = match channels.len() {
                1 => "",
                _ => "s",
            };

            format!("{} channel{}.", verb, plural)
        },
    };

    match db::upsert_channels(&channels, enabled) {
        Ok(_) => {
            let _ = utils::say(&msg.channel_id, &ctx, &message);
            refresh_disabled_channel_ids(ctx)
        },
        Err(err) => Err(CommandError::from(err)),
    }
}
