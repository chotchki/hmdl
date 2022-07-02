use std::str::FromStr;

use sqlx::{
    sqlite::{
        SqliteConnectOptions, SqliteLockingMode::Exclusive, SqlitePoolOptions,
        SqliteSynchronous::Normal,
    },
    Error, Executor, SqlitePool,
};

pub struct DatabaseHandle;

const CONNECT_DEV: &str = "sqlite::memory:";
const CONNECT_PROD: &str = "sqlite://../data/data.db";

fn database_string() -> &'static str {
    if cfg!(debug_assertions) {
        CONNECT_DEV
    } else {
        CONNECT_PROD
    }
}

impl DatabaseHandle {
    pub async fn create() -> Result<SqlitePool, Error> {
        let pool_opts = SqlitePoolOptions::new();
        let con_opts = SqliteConnectOptions::from_str(database_string())?
            .create_if_missing(true)
            .locking_mode(Exclusive)
            .shared_cache(true)
            .synchronous(Normal);

        let pool = pool_opts.connect_with(con_opts).await?;

        let mut conn = pool.acquire().await?;

        conn.execute(include_str!(
            "../../migrations/20220626_TableKnownDomains.sql"
        ))
        .await?;

        Ok(pool)
    }
}
