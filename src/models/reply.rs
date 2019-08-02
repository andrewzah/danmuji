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

    pub fn pretty_print(mut self) -> String {
        self.list.sort_by(|a, b| a.tag.to_lowercase().cmp(&b.tag.to_lowercase()));

        let tags = self.list
            .into_iter()
            .map(|r| r.tag)
            .collect::<Vec<String>>();

        tags.join(", ")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_sorts_replies() {
        let reply_list = ReplyList {
            list: vec![
                Reply { id: 0, tag: "deem".into(), url: "".into(), guild_id: "".into() },
                Reply { id: 1, tag: "deft".into(), url: "".into(), guild_id: "".into() },
                Reply { id: 2, tag: "deer".into(), url: "".into(), guild_id: "".into() },
                Reply { id: 3, tag: "defm".into(), url: "".into(), guild_id: "".into() },
            ]
        };

        assert_eq!("deem, deer, defm, deft", reply_list.pretty_print());
    }
}
