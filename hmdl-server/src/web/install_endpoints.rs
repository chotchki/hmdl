use std::{
    io,
    net::{IpAddr, Ipv6Addr, SocketAddr},
};

use axum::{extract::Host, handler::Handler, response::Redirect, BoxError, Router};
use hyper::{StatusCode, Uri};
use sqlx::SqlitePool;
use thiserror::Error;
use tokio::sync::broadcast::{error::RecvError, Receiver, Sender};

use crate::coordinator::SetupStatus;

use super::endpoints::{health, HTTPS_PORT};

pub mod setup;

pub const HTTP_PORT: u16 = 80;

pub struct InstallEndpoints {
    pool: SqlitePool,
}

impl InstallEndpoints {
    pub fn create(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn start(
        &self,
        mut install_stat_reciever: Receiver<SetupStatus>,
        install_refresh_sender: Sender<()>,
    ) -> Result<(), InstallEndpointsError> {
        tracing::debug!("Checking for setup status.");

        let mut status = install_stat_reciever.recv().await?;

        loop {
            let addr = SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), HTTP_PORT);
            let builder = axum_server::bind(addr);

            if let SetupStatus::Setup(settings) = &status {
                tracing::info!("HTTP Redirect Server listening on {}", HTTP_PORT);

                let redirect = move |Host(host): Host, uri: Uri| async move {
                    match Self::make_https(host, uri) {
                        Ok(uri) => Ok(Redirect::permanent(&uri.to_string())),
                        Err(error) => {
                            tracing::warn!(%error, "failed to convert URI to HTTPS");
                            Err(StatusCode::BAD_REQUEST)
                        }
                    }
                };
                tokio::select! {
                    Ok(()) = builder.serve(redirect.into_make_service()) => {
                        tracing::info!("HTTP Redirect Server Exited");
                    },
                    Ok(s) = install_stat_reciever.recv() => {
                        tracing::info!("Setup Status changed");
                        status = s;
                    }
                    else {
                        return(Ok(()));
                    }
                }
            } else {
                tracing::info!("HTTP Install Server listening on {}", HTTP_PORT);
                let app_service =
                    Self::create_router(self.pool.clone(), install_refresh_sender.clone());

                tokio::select! {
                    Ok(()) = builder.serve(app_service.into_make_service()) => {
                        tracing::info!("HTTP Install Server Exited");
                    },
                    Ok(s) = install_stat_reciever.recv() => {
                        tracing::info!("Setup Status changed");
                        status = s;
                    }
                    else {
                        return(Ok(()));
                    }
                }
            }
        }
    }

    fn create_router(pool: SqlitePool, install_refresh_sender: Sender<()>) -> Router {
        let mut app = Router::new().fallback(fallback.into_service());

        app = app.merge(health::router());
        app = app.merge(setup::router(pool, install_refresh_sender));

        //Only enable static content if we're in release mode
        #[cfg(not(debug_assertions))]
        {
            app = app.merge(crate::web::frontend::router());
        }

        app
    }

    fn make_https(host: String, uri: Uri) -> Result<Uri, BoxError> {
        let mut parts = uri.into_parts();

        parts.scheme = Some(axum::http::uri::Scheme::HTTPS);

        if parts.path_and_query.is_none() {
            parts.path_and_query = Some("/".parse().unwrap());
        }

        let https_host = host.replace(&HTTP_PORT.to_string(), &HTTPS_PORT.to_string());
        parts.authority = Some(https_host.parse()?);

        Ok(Uri::from_parts(parts)?)
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
    /*#[error(transparent)]
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
    SqlxError(#[from] sqlx::Error),*/
}
