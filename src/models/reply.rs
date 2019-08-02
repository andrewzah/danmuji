use diesel::prelude::*;

use crate::schema::replies;

#[derive(Queryable, PartialEq, Debug)]
pub struct Reply {
    pub id: i32,
    pub tag: String,
    pub url: String,
    pub guild_id: String,
}

#[derive(Insertable, Debug)]
#[table_name = "replies"]
pub struct NewReply<'a> {
    pub tag: &'a str,
    pub url: &'a str,
    pub guild_id: &'a str,
}

pub struct ReplyList {
    list: Vec<Reply>,
}

impl ReplyList {
    pub fn new(list: Vec<Reply>) -> ReplyList {
        ReplyList { list }
    }

    // TODO: cache/get name
    pub fn pretty_print(self) -> String {
        let tags = self.list
            .into_iter()
            .map(|r| r.tag)
            .collect::<Vec<String>>();

        tags.join(", ")
    }
}
