use chrono::{DateTime, FixedOffset, Utc};
use log::{error, info};
use serenity::model::{channel::Message as SerenityMessage, id::GuildId};

use crate::{db, errors::Result, schema::messages, utils};

#[derive(Queryable, PartialEq, Debug)]
pub struct Message {
    pub id: i32,
    pub content: String,
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

#[derive(Insertable, Clone, Debug)]
#[table_name = "messages"]
pub struct NewMessage {
    pub content: String,
    pub message_id: String,
    pub guild_id: String,
    pub channel_id: String,
    pub user_id: String,
    pub hangeul_count: i32,
    pub non_hangeul_count: i32,
    pub raw_count: i32,
    pub time: DateTime<FixedOffset>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct CharCount {
    hangeul: i32,
    non_hangeul: i32,
    raw: i32,
}

impl CharCount {
    pub fn new(hangeul: i32, non_hangeul: i32, raw: i32) -> CharCount {
        CharCount { hangeul, non_hangeul, raw }
    }
}

impl NewMessage {
    pub fn from_msg(msg: &SerenityMessage) -> Result<NewMessage> {
        let char_count = utils::parse_message_content(&msg.content)?;

        Ok(NewMessage {
            content: msg.content.clone(),
            message_id: msg.id.to_string(),
            guild_id: msg.guild_id.unwrap_or(GuildId(0_u64)).to_string(),
            channel_id: msg.channel_id.to_string(),
            user_id: msg.author.id.to_string(),
            hangeul_count: char_count.hangeul,
            non_hangeul_count: char_count.non_hangeul,
            raw_count: char_count.raw,
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
