use sqlx::SqlitePool;
use tokio::sync::broadcast::Sender;

#[derive(Clone)]
pub struct ApiContext {
    pub pool: SqlitePool,
}

#[derive(Clone)]
pub struct ApiContextSetup {
    pub pool: SqlitePool,
    pub install_refresh_sender: Sender<()>,
}
