use super::endpoints::health;
use crate::coordinator::SetupStatus;
use axum::{handler::Handler, response::Redirect, BoxError, Router};
use hyper::{StatusCode, Uri};
use sqlx::SqlitePool;
use std::{
    io,
    net::{IpAddr, Ipv6Addr, SocketAddr},
};
use thiserror::Error;
use tokio::sync::broadcast::{error::RecvError, Receiver, Sender};

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

        if matches!(status, SetupStatus::NotSetup | SetupStatus::InProgress(_)) {
            tracing::info!("HTTP Install Server listening on {}", HTTP_PORT);
            let app_service =
                Self::create_router(self.pool.clone(), install_refresh_sender.clone());

            let http_handle = tokio::spawn(async {
                let addr = SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), HTTP_PORT);
                let builder = axum_server::bind(addr);
                builder.serve(app_service.into_make_service()).await
            });

            loop {
                status = install_stat_reciever.recv().await?;
                if matches!(status, SetupStatus::Setup(_)) {
                    http_handle.abort();
                    break;
                }
            }
        }

        //Now we know the server is setup, switch to a redirect server
        if let SetupStatus::Setup(settings) = status {
            tracing::info!(
                "HTTP Redirect Server listening on {} for {}",
                HTTP_PORT,
                settings.application_domain
            );

            let host = settings.application_domain.clone();
            let redirect = move |uri: Uri| async move {
                match Self::make_https(host, uri) {
                    Ok(uri) => Ok(Redirect::permanent(&uri.to_string())),
                    Err(error) => {
                        tracing::warn!(%error, "failed to convert URI to HTTPS");
                        Err(StatusCode::BAD_REQUEST)
                    }
                }
            };

            let addr = SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), HTTP_PORT);
            let builder = axum_server::bind(addr);
            builder.serve(redirect.into_make_service()).await?;
        }
        Ok(())
    }

    fn create_router(pool: SqlitePool, install_refresh_sender: Sender<()>) -> Router {
        let mut app = Router::new().fallback(fallback.into_service());

        app = app.merge(health::router());
        app = app.merge(setup::router(pool, install_refresh_sender));

        //Only enable embedded static content if we're in release mode
        #[cfg(debug_assertions)]
        {
            app = app.merge(crate::web::dev_frontend::router());
        }
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

        parts.authority = Some(host.parse()?);

        Ok(Uri::from_parts(parts)?)
    }
}

async fn fallback() -> Redirect {
    Redirect::to("/")
}

#[derive(Debug, Error)]
pub enum InstallEndpointsError {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    Recv(#[from] RecvError),
    /*#[error(transparent)]
    HyperError(#[from] hyper::Error),



    #[error("Missing acme email")]
    MissingAcmeEmail,



    #[error(transparent)]
    RustlsError(#[from] rustls::Error),

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),*/
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_https() -> Result<(), Box<dyn std::error::Error>> {
        let test_uri = Uri::from_static("http://localhost/api/is-setup");
        let new_uri = InstallEndpoints::make_https("https.pvt".to_string(), test_uri).unwrap();

        assert_eq!("https://https.pvt/api/is-setup", new_uri.to_string());

        Ok(())
    }
}
