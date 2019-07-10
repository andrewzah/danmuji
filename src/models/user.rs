use diesel::prelude::*;

use crate::schema::users;

#[derive(Queryable, PartialEq, Debug)]
pub struct User {
    pub id: i32,
    pub user_id: String,
    pub opt_out: bool,
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub user_id: &'a str,
    pub opt_out: bool,
}
