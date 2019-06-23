use std::{env, time::Duration, sync::Arc};

use diesel::sql_query;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;
use diesel::prelude::*;
use serenity::prelude::*;
use log::{info, error};
use lazy_static::lazy_static;

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

pub struct InsertMessage {
    pub guild_id: String,
    pub channel_id: String,
    pub user_id: String,
    pub message_id: String,
    pub hangeul_count: i32,
    pub non_hangeul_count: i32,
    pub raw_count: i32,
    pub time: String,
}

// --------------------------------------


macro_rules! sql_string {
    () => (r#"
        INSERT INTO guilds (id)
        VALUES ('{guild_id}')
        ON CONFLICT DO NOTHING;

        INSERT INTO channels (id, guild_id)
        VALUES ('{channel_id}', '{guild_id}')
        ON CONFLICT DO NOTHING;

        INSERT INTO users (id)
        VALUES ('{user_id}')
        ON CONFLICT DO NOTHING;

        INSERT INTO messages
        (id, guild_id, channel_id, user_id, hangeul_count,
        non_hangeul_count, raw_count, time)
        VALUES
        ('{message_id}', '{guild_id}', '{channel_id}', '{user_id}', {hangeul_count}, {non_hangeul_count}, {raw_count}, '{time}');
    "#)
}

pub fn insert(ctx: &mut Context, im: InsertMessage) {
    let data = ctx.data.read();

    let pool = match data.get::<DbConn>() {
        Some(pool) => pool,
        None => {
            error!("There was an error getting the db connection!");
            return;
        }
    };

    let query = format!(
        sql_string!(),
        guild_id = im.guild_id,
        channel_id = im.channel_id,
        user_id = im.user_id,
        message_id = im.message_id,
        hangeul_count = im.hangeul_count,
        non_hangeul_count = im.non_hangeul_count,
        raw_count = im.raw_count,
        time = im.time
    );
    info!("{}", &query);

    let conn = pool.get().expect("Unable to get connection from pool");
    match conn.execute(&query) {
        Ok(res) => info!("inserted successfully: {}", res),
        Err(err) => error!("error inserting: {}", err)
    }
}
