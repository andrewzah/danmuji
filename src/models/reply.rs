use diesel::prelude::*;

use crate::schema::replies;

#[derive(Queryable, PartialEq, Debug)]
pub struct Reply {
    pub id: i32,
    pub tag: String,
    pub url: String,
}

#[derive(Insertable, Debug)]
#[table_name = "replies"]
pub struct NewReply<'a> {
    pub tag: &'a str,
    pub url: &'a str,
}
