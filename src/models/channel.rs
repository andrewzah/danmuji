use std::{thread, time};

use serenity::{model::{prelude::Message, channel::ChannelType}, http::raw::Http};

use crate::{errors::*, schema::channels};

#[derive(Queryable, PartialEq, Debug)]
pub struct Channel {
    pub id: i32,
    pub channel_id: String,
    pub enabled: bool,
}

// TODO: is changeset needed?
#[derive(AsChangeset, Insertable, Debug)]
#[table_name = "channels"]
pub struct NewChannel {
    pub channel_id: String,
    pub enabled: bool,
}

#[derive(Queryable, PartialEq, Debug)]
pub struct ChannelId {
    pub id: i32,
    pub channel_id: String,
}

pub struct ChannelList {
    list: Vec<Channel>,
}

impl ChannelList {
    pub fn new(list: Vec<Channel>) -> ChannelList {
        ChannelList { list }
    }

    // TODO: cache/get name
    pub fn pretty_print(&self, msg: &Message, http: &Http) -> Result<String> {
        let mut channel_names: Vec<String> = vec![];

        for channel in &self.list {
            let _ = &msg.channel_id.broadcast_typing(&http);
            let channel_id = channel.channel_id.parse::<u64>()?;

            if let Ok(channel) = http.get_channel(channel_id) {
                if let Some(guild_lock) = channel.guild() {
                    let guild_channel = guild_lock.read();
                    if guild_channel.kind == ChannelType::Text {
                        channel_names.push(guild_channel.name.clone());
                    }
                }
            };

            thread::sleep(time::Duration::from_millis(500));
        }

        channel_names.sort();

        Ok(channel_names.join("\n"))
    }
}
