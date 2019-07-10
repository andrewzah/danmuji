use diesel::prelude::*;

use crate::schema::channels;

#[derive(Queryable, PartialEq, Debug)]
pub struct Channel {
    pub id: i32,
    pub channel_id: String,
    pub enabled: bool,
}

#[derive(Insertable, Debug)]
#[table_name = "channels"]
pub struct NewChannel<'a> {
    pub channel_id: &'a str,
    pub enabled: bool,
}
