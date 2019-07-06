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

use crate::{errors::*, models::*, schema::messages};

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

pub fn insert_message(ctx: &Context, nm: NewMessage) -> Result<usize> {
    use crate::schema::messages::dsl::*;

    let data = ctx.data.read();
    let pool = match data.get::<DbPool>() {
        Some(pool) => pool,
        None => return Err(AppError::from_string("A")),
    };

    let conn = pool.get()?;
    insert_into(messages)
        .values(&nm)
        .execute(&conn)
        .map_err(|err| AppError::new(ErrorKind::DbResult(err)))
}

pub fn get_ratio_list() -> Result<RatioResultList> {
    use crate::schema::messages::dsl::*;

    let conn = pool().get()?;
    let sql = fs::read_to_string("sql/ratio.sql")?;

    let results = sql_query(sql).load(&conn);

    results
        .map(|list| RatioResultList::from_list(list))
        .map_err(|err| AppError::new(ErrorKind::DbResult(err)))
}
