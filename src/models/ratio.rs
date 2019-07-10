use std::{thread, time};

use serenity::http::raw::Http;
use diesel::{prelude::*, sql_types::*};

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
    pub ratio: f64,
}

pub struct RatioResultList {
    list: Vec<RatioResult>,
}

impl RatioResultList {
    pub fn from_list(list: Vec<RatioResult>) -> RatioResultList {
        RatioResultList { list }
    }

    pub fn pretty_print(&self, http: &Http) -> String {
        let mut result = String::new();
        result.push_str("Ratio Results:\n");

        for rr in &self.list {
            //#TODO eliminate unwrap, probably by using u64 in db, not string
            //
            let name = match http.get_user(rr.user_id.parse::<u64>().unwrap()) {
                Ok(user) => user.name,
                Err(_) => "Err grabbing name!".into(),
            };

            let s = format!(
                " + {}: ratio: {}, total-messages: {}, hangeul: {}, non-hangeul: {}, raw-count: {}\n",
                name, rr.ratio, rr.sum_messages, rr.sum_hangeul_count,
                rr.sum_non_hangeul_count, rr.sum_raw_count
            );
            result.push_str(&s);

            thread::sleep(time::Duration::from_secs(1));
        }

        result
    }
}
