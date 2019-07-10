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
    models::{channel::NewChannel, message::NewMessage, ratio::RatioResultList, user::NewUser},
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

pub fn insert_channel(nc: NewChannel) -> Result<usize> {
    use crate::schema::channels::dsl::*;

    let conn = pool().get()?;
    insert_into(channels)
        .values(&nc)
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

// ---------------- SQL QUERIES ----------------------

pub fn get_ratio_list() -> Result<RatioResultList> {
    use crate::schema::messages::dsl::*;

    let conn = pool().get()?;
    let sql = fs::read_to_string("sql/ratio.sql")?;

    let results = sql_query(sql).load(&conn);

    results
        .map(|list| RatioResultList::from_list(list))
        .map_err(|err| AppError::new(ErrorKind::DbResult(err)))
}
