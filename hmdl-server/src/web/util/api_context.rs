use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::sync::broadcast::Sender;
use webauthn_rs::Webauthn;

#[derive(Clone)]
pub struct ApiContext {
    pub pool: SqlitePool,
}

#[derive(Clone)]
pub struct ApiContextSetup {
    pub pool: SqlitePool,
    pub install_refresh_sender: Sender<()>,
}

#[derive(Clone)]
pub struct ApiContextAuth {
    pub pool: SqlitePool,
    pub webauthn: Arc<Webauthn>,
}
