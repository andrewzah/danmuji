use std::{
    env,
    fs,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use diesel::{
    insert_into,
    prelude::*,
    r2d2::{self, ConnectionManager},
    sql_query,
};
use lazy_static::lazy_static;
use log::info;
use serenity::prelude::*;

use crate::{
    errors::*,
    models::{
        channel::{Channel, ChannelId, ChannelList, NewChannel},
        message::NewMessage,
        ratio::RatioResultList,
        user::NewUser,
        reply::{NewReply, Reply},
    },
    schema::messages,
};

type Pool = Arc<r2d2::Pool<ConnectionManager<PgConnection>>>;
type Conn = r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

lazy_static! {
    static ref POOL: Pool = init_pool(&database_url());
}

pub struct DbPool;
impl TypeMapKey for DbPool {
    type Value = Pool;
}

fn database_url() -> String {
    env::var("DANMUJI_DATABASE_URL").expect("Is DATABASE_URL set?")
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

pub fn pool() -> Pool {
    Arc::clone(&POOL)
}

// ---------------- INSERTS/UPDATES --------------------

pub fn insert_message(nm: NewMessage) -> Result<usize> {
    use crate::schema::messages::dsl::*;

    let conn = pool().get()?;
    insert_into(messages)
        .values(&nm)
        .execute(&conn)
        .map_err(|err| AppError::new(ErrorKind::DbResult(err)))
}

pub fn upsert_user(nu: &NewUser) -> Result<usize> {
    use crate::schema::users::dsl::*;

    let conn = pool().get()?;
    insert_into(users)
        .values(nu)
        .on_conflict(user_id)
        .do_update()
        .set(opt_out.eq(nu.opt_out))
        .execute(&conn)
        .map_err(|err| AppError::new(ErrorKind::DbResult(err)))
}

pub fn upsert_channel(nc: &NewChannel) -> Result<usize> {
    use crate::schema::channels::dsl::*;

    let conn = pool().get()?;
    insert_into(channels)
        .values(nc)
        .on_conflict(channel_id)
        .do_update()
        .set(enabled.eq(nc.enabled))
        .execute(&conn)
        .map_err(|err| AppError::new(ErrorKind::DbResult(err)))
}

// TODO: rename new_etc to just etc, since it's an `upsert`...
pub fn upsert_channels(ncs: &Vec<NewChannel>, new_enabled: bool) -> Result<usize> {
    use crate::schema::channels::dsl::*;

    let conn = pool().get()?;
    insert_into(channels)
        .values(ncs)
        .on_conflict(channel_id)
        .do_update()
        .set(enabled.eq(new_enabled))
        .execute(&conn)
        .map_err(|err| AppError::new(ErrorKind::DbResult(err)))
}

pub fn all_channels() -> Result<ChannelList> {
    use crate::schema::channels::dsl::*;

    let conn = pool().get()?;
    channels
        .load::<Channel>(&conn)
        .map(|list| ChannelList::new(list))
        .map_err(|e| AppError::new(ErrorKind::DbResult(e)))
}

pub fn disabled_channel_ids() -> Result<Vec<u64>> {
    use crate::schema::channels::dsl::*;

    let conn = pool().get()?;
    let results = channels
        .select(channel_id)
        .filter(enabled.eq(false))
        .load::<String>(&conn)
        .map_err(|e| AppError::new(ErrorKind::DbResult(e)));

    match results {
        Ok(c_ids) => {
            let ids = c_ids
                .into_iter()
                .map(|c_id| c_id.parse::<u64>().expect("Unable to parse channel id!"))
                .collect();
            info!("ids: {:?}", &ids);
            Ok(ids)
        },
        Err(err) => Err(err),
    }
}

pub fn get_reply(input: &str) -> Result<Reply> {
    use crate::schema::replies::dsl::*;

    let conn = pool().get()?;
    replies
        .filter(tag.eq(input))
        .get_result::<Reply>(&conn)
        .map_err(|e| AppError::new(ErrorKind::DbResult(e)))
}

pub fn upsert_reply(reply: &NewReply) -> Result<usize> {
    use crate::schema::replies::dsl::*;

    let conn = pool().get()?;
    insert_into(replies)
        .values(reply)
        .on_conflict(tag)
        .do_update()
        .set(url.eq(reply.url))
        .execute(&conn)
        .map_err(|err| AppError::new(ErrorKind::DbResult(err)))
}

// ---------------- SQL QUERIES ----------------------

pub fn get_ratio_list() -> Result<RatioResultList> {
    use crate::schema::messages::dsl::*;

    let conn = pool().get()?;
    let sql = fs::read_to_string("sql/ratio.sql")?;

    let results = sql_query(sql).load(&conn);

    results
        .map(|list| RatioResultList::new(list))
        .map_err(|err| AppError::new(ErrorKind::DbResult(err)))
}
