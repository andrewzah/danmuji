use std::{env, time::Duration, sync::Arc};

use chrono::{DateTime, FixedOffset};
use diesel::insert_into;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;
use diesel::prelude::*;
use serenity::prelude::*;
use log::{info};
use lazy_static::lazy_static;

use crate::errors::*;
use crate::schema::messages;

type Pool = Arc<r2d2::Pool<ConnectionManager<PgConnection>>>;

lazy_static!{
    static ref POOL: Pool = init_pool(&database_url());
}

pub struct DbConn;
impl TypeMapKey for DbConn {
    type Value = Pool;
}

fn database_url() -> String {
    env::var("DATABASE_URL").expect("Is DATABASE_URL set?")
}

fn init_pool(db_url: &str) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let r2d2_pool = r2d2::Pool::builder()
        .max_size(1)
        .connection_timeout(Duration::from_secs(60))
        .build(manager)
        .expect("Unable to build pool!");

    let pool = Arc::new(r2d2_pool);
    info!("Database pool initialized!");

    pool
}

pub fn connection() -> Pool {
    Arc::clone(&POOL)
}

#[derive(Insertable, Debug)]
#[table_name = "messages"]
pub struct NewMessage<'a> {
    pub message_id: &'a str,
    pub guild_id: &'a str,
    pub channel_id: &'a str,
    pub user_id: &'a str,
    pub hangeul_count: i32,
    pub non_hangeul_count: i32,
    pub raw_count: i32,
    pub time: DateTime<FixedOffset>,
}

#[derive(Queryable, PartialEq, Debug)]
pub struct Message {
    pub id: String,
    pub message_id: String,
    pub guild_id: String,
    pub channel_id: String,
    pub user_id: String,
    pub hangeul_count: i32,
    pub non_hangeul_count: i32,
    pub raw_count: i32,
    pub time: DateTime<FixedOffset>,
}

// --------------------------------------

pub fn insert_message(ctx: &mut Context, nm: NewMessage) -> Result<usize> {
    use crate::schema::messages::dsl::*;
    let data = ctx.data.read();

    let pool = match data.get::<DbConn>() {
        Some(pool) => pool,
        None => return Err(AppError::from_string("A"))
    };

    let conn = pool.get()?;
    insert_into(messages).values(&nm).execute(&conn)
        .map_err(|err| AppError::new(ErrorKind::DbResult(err)))
}
