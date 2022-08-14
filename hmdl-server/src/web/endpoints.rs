use std::{
    io,
    net::{IpAddr, Ipv6Addr, SocketAddr},
};

use axum::{handler::Handler, http::StatusCode, Router};
use axum_server::tls_rustls::RustlsConfig;
use sqlx::{query, SqlitePool};
use thiserror::Error;
use tokio::sync::broadcast::{error::RecvError, Receiver};

pub mod client_groups;
pub mod clients;
pub mod domain_groups;
pub mod domains;
pub mod groups_applied;
pub mod health;

pub const HTTPS_PORT: u16 = 443;

pub struct Endpoints {
    pool: SqlitePool,
}

impl Endpoints {
    pub fn create(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn start(
        &self,
        mut tls_config_reciever: Receiver<RustlsConfig>,
    ) -> Result<(), EndpointsError> {
        let config = tls_config_reciever.recv().await?;

        let addr = SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), HTTPS_PORT);
        let app_serv = self.create_router().into_make_service();
        let builder = axum_server::bind_rustls(addr, config);

        //Update that we are starting the https server
        let mut conn = self.pool.acquire().await?;
        query!(
            r#"
            UPDATE hmdl_settings
            SET https_started_once=true
            WHERE lock_column=true
            "#
        )
        .execute(&mut conn)
        .await?;

        tracing::info!("HTTPS Server listening on {}", addr);
        builder.serve(app_serv).await?;

        Ok(())
    }

    fn create_router(&self) -> Router {
        let mut app = Router::new().fallback(fallback.into_service());

        app = app.merge(clients::router(self.pool.clone()));
        app = app.merge(client_groups::router(self.pool.clone()));
        app = app.merge(domains::router(self.pool.clone()));
        app = app.merge(domain_groups::router(self.pool.clone()));
        app = app.merge(groups_applied::router(self.pool.clone()));
        app = app.merge(health::router());

        //Only enable static content if we're in release mode
        #[cfg(not(debug_assertions))]
        {
            app = app.merge(crate::web::frontend::router());
        }

        app
    }
}

#[derive(Debug, Error)]
pub enum EndpointsError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Recv(#[from] RecvError),
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
}

async fn fallback() -> (StatusCode, String) {
    (
        StatusCode::NOT_FOUND,
        "404 - Yeah you're not finding what you want.".to_string(),
    )
}
