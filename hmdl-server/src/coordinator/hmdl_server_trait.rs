use std::net::IpAddr;

use async_trait::async_trait;
use sqlx::SqlitePool;
use thiserror::Error;
use tokio::sync::mpsc::UnboundedReceiver;

#[async_trait]
pub trait HmdlServerTrait {
    fn create(pool: SqlitePool, shutdown_hook: UnboundedReceiver<()>) -> Self;

    async fn start_on(addr: IpAddr) -> Result<(), HmdlServerError>;
}

#[derive(Debug, Error)]
pub enum HmdlServerError {}
