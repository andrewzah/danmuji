use chrono::{DateTime, FixedOffset, Utc};
use diesel::{sql_types::*, prelude::*};
use log::{info,error};
use serenity::{client::Context, model::{channel::Message as SerenityMessage, id::GuildId}};

use crate::{db, errors::Result, schema::messages, utils};

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
    pub fn from_msg(msg: SerenityMessage) -> Result<NewMessage> {
        let (hc, nhc, rc) = utils::parse_content(&msg.content)?;
        Ok(
        NewMessage {
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

    pub fn write_to_db(self, ctx: &Context) {
        match db::insert_message(&ctx, self) {
            Ok(u) => info!("finished inserting msg: usize? {}", u),
            Err(err) => error!(":x: error: {}", err),
        }
    }
}

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
pub struct UserId {
    pub user_id: String,
}

#[derive(QueryableByName, PartialEq, Debug)]
pub struct RatioResult {
    #[sql_type = "Text"]
    pub user_id: String,

    #[sql_type = "Integer"]
    pub sum_hangeul_count: i32,

    #[sql_type = "Integer"]
    pub sum_non_hangeul_count: i32,

    #[sql_type = "Integer"]
    pub sum_raw_count: i32,

    #[sql_type = "BigInt"]
    pub sum_messages: i64,

    #[sql_type = "Double"]
    pub ratio: f64
}

pub struct RatioResultList {
    list: Vec<RatioResult>
}

impl RatioResultList {
    pub fn from_list(list: Vec<RatioResult>) -> RatioResultList {
        RatioResultList { list }
    }

    pub fn pretty_print(&self) -> String {
        let mut result = String::new();
        result.push_str("Ratio Results:\n");

        for rr in &self.list {
            let s = format!(
                " + {}: ratio: {}, total-messages: {}, hangeul: {}, non-hangeul: {}, raw-count: {}\n",
                rr.user_id, rr.ratio, rr.sum_messages, rr.sum_hangeul_count,
                rr.sum_non_hangeul_count, rr.sum_raw_count
            );
            result.push_str(&s);
        }

        result
    }
}
