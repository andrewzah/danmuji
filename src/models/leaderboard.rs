use std::{thread, time};

use diesel::sql_types::*;
use serenity::http::raw::Http;

use crate::errors::Result;

#[derive(QueryableByName, PartialEq, Debug)]
pub struct LeaderboardEntry {
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
    pub ratio: f64,
}

pub struct LeaderBoard {
    list: Vec<LeaderboardEntry>,
}

impl LeaderBoard {
    pub fn new(list: Vec<LeaderboardEntry>) -> LeaderBoard {
        LeaderBoard { list }
    }

    // TODO: cache
    pub fn pretty_print(&self, http: &Http) -> Result<String> {
        let mut result = String::new();
        result.push_str("The top 5 users for this month are:\n\n");

        for entry in &self.list {
            let user_id = entry.user_id.parse::<u64>()?;
            let name = http.get_user(user_id)?;

            let s = format!(
                "* {}: **Ratio**: {:.2}%, **Hangeul**: {}, **Non-Hangeul**: {}, **Total Messages**: {}, **Raw Count**: {}\n",
                name, entry.ratio, entry.sum_hangeul_count,
                entry.sum_non_hangeul_count, entry.sum_messages, entry.sum_raw_count
            );
            result.push_str(&s);

            thread::sleep(time::Duration::from_millis(550));
        }

        result.push_str("\nThe script that calculates these results can be found here: <https://git.sr.ht/~andrewzah/danmuji/tree/master/sql/leaderboard.sql>. Let <@91329651909074944> know if there are any issues.");

        Ok(result)
    }
}
