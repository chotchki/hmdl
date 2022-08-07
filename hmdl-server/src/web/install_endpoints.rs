use std::{
    collections::HashSet,
    net::{IpAddr, SocketAddr},
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

const PORT: u16 = 8443;

pub struct InstallEndpoints {
    pool: SqlitePool,
}

impl InstallEndpoints {
    pub fn create(pool: SqlitePool) -> Self {
        InstallEndpoints { pool }
    }

    pub async fn start(
        &self,
        mut ip_changed: Receiver<HashSet<IpAddr>>,
        settings_changed: Sender<HashSet<IpAddr>>,
    ) -> Result<(), InstallEndpointsError> {
        let mut ips = ip_changed.recv().await?;

        loop {
            let mut handles = JoinSet::new();

            //Filtering down to a single example to figure out the error
            //ips = ips
            //    .into_iter()
            //    .filter(|x| x.is_loopback() && x.is_ipv6())
            //    .collect();

            for ip in ips {
                tracing::info!("HTTP Install Server listening on {}", ip);
                let addr = SocketAddr::new(ip, PORT);
                let app_service = Self::create_router(self.pool.clone()).into_make_service();

                let builder = axum_server::bind(addr);
                handles.spawn(async move { builder.serve(app_service).await });
            }

            tokio::select! {
                Some(res) = handles.join_one() => {
                    tracing::debug!("Tokio task ended {:#?}, cancelling the rest", res);

                    handles.abort_all();
                    while !handles.is_empty() {
                        handles.join_one().await;
                    }

                    break;
                },
                Ok(new_ips) = ip_changed.recv() => {
                    tracing::info!("Recieved Install change, restarting DNS server");
                    ips = new_ips;
                },
                else => {
                    tracing::warn!("Futures aborted, shutting down install server");
                    break;
                }
            };
        }

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

    #[error("Missing acme email")]
    MissingAcmeEmail,

    #[error(transparent)]
    Recv(#[from] RecvError),

    #[error(transparent)]
    RustlsError(#[from] rustls::Error),

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
}
