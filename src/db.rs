use std::{collections::HashMap, env, fs, sync::Arc, time::Duration};

use diesel::{
    insert_into,
    prelude::*,
    r2d2::{self, ConnectionManager},
    sql_query,
};
use lazy_static::lazy_static;
use serenity::prelude::*;
use strfmt::strfmt;

use crate::{
    errors::*,
    models::{
        channel::{Channel, ChannelList, NewChannel},
        message::NewMessage,
        leaderboard::LeaderBoard,
        reply::{NewReply, Reply, ReplyList},
        user::NewUser,
    },
};

type Pool = Arc<r2d2::Pool<ConnectionManager<PgConnection>>>;

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
        .max_size(30)
        .idle_timeout(Some(Duration::from_secs(30)))
        .connection_timeout(Duration::from_secs(30))
        .build(manager)
        .expect("Unable to build pool!");

    let pool = Arc::new(r2d2_pool);

    pool
}

pub fn pool() -> Pool {
    Arc::clone(&POOL)
}

// ---------------- CRUD --------------------

pub fn insert_message(nm: NewMessage) -> Result<usize> {
    use crate::schema::messages::dsl::*;

    let conn = pool().get()?;
    insert_into(messages)
        .values(&nm)
        .execute(&conn)
        .map_err(|err| AppError::new(ErrorKind::DbResult(err)))
}

pub fn insert_messages(nms: &Vec<NewMessage>) -> Result<usize> {
    use crate::schema::messages::dsl::*;

    let conn = pool().get()?;
    insert_into(messages)
        .values(nms)
        .execute(&conn)
        .map_err(|err| AppError::new(ErrorKind::DbResult(err)))
}

pub fn delete_guild_messages(gid: &str) -> Result<usize> {
    use crate::schema::messages::dsl::*;

    let conn = pool().get()?;
    diesel::delete(messages.filter(guild_id.eq(gid)))
        .execute(&conn)
        .map_err(|e| AppError::new(ErrorKind::DbResult(e)))
}

pub fn delete_all_messages() -> Result<usize> {
    use crate::schema::messages::dsl::*;

    let conn = pool().get()?;
    diesel::delete(messages)
        .execute(&conn)
        .map_err(|e| AppError::new(ErrorKind::DbResult(e)))
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
            Ok(ids)
        },
        Err(err) => Err(err),
    }
}

pub fn get_replies(gid: &str) -> Result<ReplyList> {
    use crate::schema::replies::dsl::*;

    let conn = pool().get()?;
    replies
        .filter(guild_id.eq(gid))
        .load::<Reply>(&conn)
        .map(|list| ReplyList::new(list))
        .map_err(|e| AppError::new(ErrorKind::DbResult(e)))
}

pub fn get_reply(keyword: &str, gid: &str) -> Result<Reply> {
    use crate::schema::replies::dsl::*;

    let conn = pool().get()?;
    replies
        .filter(guild_id.eq(gid).and(tag.eq(keyword)))
        .get_result::<Reply>(&conn)
        .map_err(|e| AppError::new(ErrorKind::DbResult(e)))
}

pub fn upsert_reply(reply: &NewReply) -> Result<usize> {
    use crate::schema::replies::dsl::*;

    let conn = pool().get()?;
    insert_into(replies)
        .values(reply)
        .on_conflict((guild_id, tag))
        .do_update()
        .set(url.eq(reply.url))
        .execute(&conn)
        .map_err(|err| AppError::new(ErrorKind::DbResult(err)))
}

pub fn delete_reply(keyword: &str, gid: &str) -> Result<usize> {
    use crate::schema::replies::dsl::*;

    let conn = pool().get()?;
    diesel::delete(replies.filter(guild_id.eq(gid).and(tag.eq(keyword))))
        .execute(&conn)
        .map_err(|e| AppError::new(ErrorKind::DbResult(e)))
}

// ---------------- SQL QUERIES ----------------------

pub fn get_leaderboard(gid: &str) -> Result<LeaderBoard> {
    let conn = pool().get()?;
    let sql_file = fs::read_to_string("sql/leaderboard.sql")?;
    let mut vars = HashMap::new();
    vars.insert("guild_id".to_string(), gid);
    let sql = strfmt(&sql_file, &vars)?;

    let results = sql_query(sql).load(&conn);

    results
        .map(|list| LeaderBoard::new(list))
        .map_err(|err| AppError::new(ErrorKind::DbResult(err)))
}
