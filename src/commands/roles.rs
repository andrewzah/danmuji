use serenity::{
    client::Context,
    framework::standard::{
        Args,
        macros::{command, group},
        CommandError,
        CommandResult,
    },
    model::channel::Message,
    utils::{parse_role, Colour},
};

use crate::{checks::*, db, models::role::{NewRole, Role}, utils};

group!({
    name: "roles",
    options: {
        allowed_roles: [
            "Mod", "Moderator", "Admin", "Administrator",
        ],
        prefixes: ["roles"],
    },
    commands: [
        enable, disable
    ],
});


#[command]
#[only_in(guilds)]
pub fn enable(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();
    let roles: Vec<&str> = args.rest().split(" ").collect();

    match roles.len() {
        1 => {
            let role_id = match parse_role(roles[0]) {
                Some(id) => id,
                None => {
                    let message = format!("Unable to parse role: {}", roles[0]);
                    return utils::say(&msg.channel_id, &ctx, &message)
                }
            };

            let new_role = NewRole {
                guild_id: guild_id.to_string(),
                role_id: msg.channel_id.to_string(),
                enabled: true,
            };

            match db::upsert_roles(&vec![new_role], true) {
                Ok(_) => utils::say(&msg.channel_id, &ctx, "Enabled role."),
                Err(err) => Err(CommandError::from(err))
            }
        },
        _ => {
            let new_roles: Vec<NewRole> = roles
                .into_iter()
                .filter_map(|role| parse_role(role))
                .map(|rid| NewRole {
                    guild_id: guild_id.to_string(),
                    role_id: rid.to_string(),
                    enabled: true,
                })
                .collect();

            match db::upsert_roles(&new_roles, true) {
                Ok(count) => utils::say(&msg.channel_id, &ctx,
                    &format!("Enabled {} roles.", count)),
                Err(err) => Err(CommandError::from(err))
            }
        },
    }
}

#[command]
#[only_in(guilds)]
pub fn disable(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();
    let roles: Vec<&str> = args.rest().split(" ").collect();

    match roles.len() {
        1 => {
            let role_id = match parse_role(roles[0]) {
                Some(id) => id,
                None => {
                    let message = format!("Unable to parse role: {}", roles[0]);
                    return utils::say(&msg.channel_id, &ctx, &message)
                }
            };

            let new_role = NewRole {
                guild_id: guild_id.to_string(),
                role_id: msg.channel_id.to_string(),
                enabled: true,
            };

            match db::upsert_roles(&vec![new_role], false) {
                Ok(_) => utils::say(&msg.channel_id, &ctx, "Disabled role."),
                Err(err) => Err(CommandError::from(err))
            }
        },
        _ => {
            let new_roles: Vec<NewRole> = roles
                .into_iter()
                .filter_map(|role| parse_role(role))
                .map(|rid| NewRole {
                    guild_id: guild_id.to_string(),
                    role_id: rid.to_string(),
                    enabled: false,
                })
                .collect();

            match db::upsert_roles(&new_roles, false) {
                Ok(count) => utils::say(&msg.channel_id, &ctx,
                    &format!("Disabled {} roles.", count)),
                Err(err) => Err(CommandError::from(err))
            }
        },
    }
}
