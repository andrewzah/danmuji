use crate::schema::roles;

#[derive(Queryable, PartialEq, Debug)]
pub struct Role {
    pub id: i32,
    pub guild_id: String,
    pub role_id: String,
    pub enabled: bool,
}

#[derive(Insertable, Debug)]
#[table_name = "roles"]
pub struct NewRole<'a> {
    pub guild_id: &'a str,
    pub role_id: &'a str,
    pub enabled: bool,
}
