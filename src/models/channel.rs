use std::{thread, time};

use serenity::http::raw::Http;

use crate::schema::channels;

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
    pub fn pretty_print(&self, _http: &Http) -> String {
        let mut result = String::new();
        result.push_str("Ratio Results:\n");

        for channel in &self.list {
            let _channel_id = channel
                .channel_id
                .parse::<u64>()
                .expect("Unable to parse channel!");
            // http.get_channel(channel_id).expect("Unable to get channel!").name,
            let s = format!(
                "**{}** - enabled: {}\n",
                channel.channel_id, channel.enabled
            );
            result.push_str(&s);

            thread::sleep(time::Duration::from_secs(1));
        }

        result
    }
}
