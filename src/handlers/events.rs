use log::{debug, error, info};
use serenity::model::{channel::Message, event::ResumedEvent, gateway::Ready, id::GuildId};
use serenity::prelude::*;

use crate::{db, utils};

const BOT_ID: u64 = 592184706896756736;

pub struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }

    fn message(&self, mut ctx: Context, msg: Message) {
        if msg.author.id == BOT_ID {
            return;
        }

        let m = db::NewMessage {
            message_id: &msg.id.to_string(),
            guild_id: &msg.guild_id.unwrap_or(GuildId(0_u64)).to_string(),
            channel_id: &msg.channel_id.to_string(),
            user_id: &msg.author.id.to_string(),
            hangeul_count: 0,
            non_hangeul_count: 1,
            raw_count: 2,
            time: msg.timestamp,
        };

        //parse_content(&formatted_content);
        match db::insert_message(&mut ctx, m) {
            Ok(u) => info!("finished inserting msg: usize? {}", u),
            Err(err) => error!(":x: error: {}", err),
        }
    }

    fn resume(&self, _: Context, resume: ResumedEvent) {
        debug!("Resumed; trace: {:?}", resume.trace);
    }
}

#[allow(dead_code)]
fn parse_content(content: &str) {
    let mut non_hangeul = 0;
    let mut hangeul = 0;

    let blocks = content.trim().split("");
    for block in blocks {
        for character in block.chars() {
            if utils::is_hangeul(character) {
                hangeul += 1;
            } else {
                non_hangeul += 1;
            }
        }
    }
}
