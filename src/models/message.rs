use chrono::{DateTime, FixedOffset, Utc};
use diesel::prelude::*;
use log::{error, info};
use serenity::{
    client::Context,
    model::{channel::Message as SerenityMessage, id::GuildId},
};

use crate::{db, errors::Result, schema::messages, utils};

#[derive(Queryable, PartialEq, Debug)]
pub struct Message {
    pub id: i32,
    pub message_id: String,
    pub guild_id: String,
    pub channel_id: String,
    pub user_id: String,
    pub hangeul_count: i32,
    pub non_hangeul_count: i32,
    pub raw_count: i32,
    pub time: DateTime<Utc>,
}

#[derive(QueryableByName, PartialEq, Debug)]
#[table_name = "messages"]
pub struct MessageUserId {
    pub user_id: String,
}

#[derive(Insertable, Debug)]
#[table_name = "messages"]
pub struct NewMessage {
    pub message_id: String,
    pub guild_id: String,
    pub channel_id: String,
    pub user_id: String,
    pub hangeul_count: i32,
    pub non_hangeul_count: i32,
    pub raw_count: i32,
    pub time: DateTime<FixedOffset>,
}

impl NewMessage {
    pub fn from_msg(msg: &SerenityMessage) -> Result<NewMessage> {
        let (hc, nhc, rc) = utils::parse_content(&msg.content)?;

        Ok(NewMessage {
            message_id: msg.id.to_string(),
            guild_id: msg.guild_id.unwrap_or(GuildId(0_u64)).to_string(),
            channel_id: msg.channel_id.to_string(),
            user_id: msg.author.id.to_string(),
            hangeul_count: hc,
            non_hangeul_count: nhc,
            raw_count: rc,
            time: msg.timestamp,
        })
    }

    pub fn insert(self) {
        match db::insert_message(self) {
            Ok(u) => info!("finished inserting msg: usize? {}", u),
            Err(err) => error!(":x: error: {}", err),
        }
    }
}
