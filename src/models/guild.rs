use crate::schema::guilds;

#[derive(Queryable, PartialEq, Debug)]
pub struct Guild {
    pub id: i32,
    pub guild_id: String,
    pub enabled: bool,
}

#[derive(Insertable, Debug)]
#[table_name = "guilds"]
pub struct NewGuild<'a> {
    pub guild_id: &'a str,
    pub enabled: bool,
}
