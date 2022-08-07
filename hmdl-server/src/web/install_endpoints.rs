use std::{
    collections::HashSet,
    io,
    net::{IpAddr, Ipv6Addr, SocketAddr},
};

use axum::{handler::Handler, Router};
use futures::future;
use hyper::StatusCode;
use sqlx::SqlitePool;
use thiserror::Error;
use tokio::{
    sync::broadcast::{error::RecvError, Receiver, Sender},
    task::JoinSet,
};

use super::endpoints::health;

pub mod setup;

const PORT: u16 = 80;

pub struct InstallEndpoints {
    pool: SqlitePool,
}

impl InstallEndpoints {
    pub fn create(pool: SqlitePool) -> Self {
        InstallEndpoints { pool }
    }

    pub async fn start(
        &self,
        settings_changed: Sender<HashSet<IpAddr>>,
    ) -> Result<(), InstallEndpointsError> {
        tracing::info!("HTTP Install Server listening on {}", PORT);

        let addr = SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), PORT);
        let app_service = Self::create_router(self.pool.clone()).into_make_service();

        let builder = axum_server::bind(addr);
        builder.serve(app_service).await?;

        Ok(())
    }

    fn create_router(pool: SqlitePool) -> Router {
        let mut app = Router::new().fallback(fallback.into_service());

        app = app.merge(health::router());
        app = app.merge(setup::router(pool));

        //Only enable static content if we're in release mode
        #[cfg(not(debug_assertions))]
        {
            app = app.merge(crate::web::frontend::router());
        }

        app
    }
}

async fn fallback() -> (StatusCode, String) {
    (
        StatusCode::NOT_FOUND,
        "404 - Yeah you're not finding what you want.".to_string(),
    )
}

#[derive(Debug, Error)]
pub enum InstallEndpointsError {
    #[error(transparent)]
    HyperError(#[from] hyper::Error),

    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("Missing acme email")]
    MissingAcmeEmail,

    #[error(transparent)]
    Recv(#[from] RecvError),

    #[error(transparent)]
    RustlsError(#[from] rustls::Error),

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
}
