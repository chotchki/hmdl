use sqlx::{Error, Executor, SqlitePool};

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
        let pool = SqlitePool::connect(database_string()).await?;

        let mut conn = pool.acquire().await?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS known_domains( \
            name text NOT NULL, \
            user_group text NOT NULL, \
            needs_review boolean NOT NULL, \
            block boolean NOT NULL, \
            PRIMARY KEY (name, user_group)
        );",
        )
        .await?;

        Ok(pool)
    }
}
