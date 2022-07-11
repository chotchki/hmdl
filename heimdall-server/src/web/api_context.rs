use sqlx::SqlitePool;

#[derive(Clone)]
pub struct ApiContext {
    pub pool: SqlitePool,
}
