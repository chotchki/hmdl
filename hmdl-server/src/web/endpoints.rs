use crate::coordinator::HmdlSetup;
use axum::{handler::Handler, response::Redirect, Router};
use axum_server::tls_rustls::RustlsConfig;
use axum_sessions::{async_session::MemoryStore, SessionLayer};
use ring::{
    error::Unspecified,
    rand::{SecureRandom, SystemRandom},
};
use sqlx::{query, SqlitePool};
use std::{
    io,
    net::{IpAddr, Ipv6Addr, SocketAddr},
    sync::Arc,
};
use thiserror::Error;
use tokio::sync::broadcast::{error::RecvError, Receiver};
use url::{ParseError, Url};
use webauthn_rs::{prelude::WebauthnError, Webauthn, WebauthnBuilder};

pub mod authentication;
pub mod client_groups;
pub mod clients;
pub mod domain_groups;
pub mod domains;
pub mod groups_applied;
pub mod health;
pub mod setup;

pub const HTTPS_PORT: u16 = 443;

pub struct Endpoints {
    pool: SqlitePool,
    rand_gen: SystemRandom,
}

impl Endpoints {
    pub fn create(pool: SqlitePool, rand_gen: SystemRandom) -> Result<Self, EndpointsError> {
        Ok(Self { pool, rand_gen })
    }

    pub async fn start(
        &self,
        mut tls_config_reciever: Receiver<(RustlsConfig, HmdlSetup)>,
    ) -> Result<(), EndpointsError> {
        let (config, setup) = tls_config_reciever.recv().await?;

        let addr = SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), HTTPS_PORT);

        let mut secret: [u8; 64] = [0; 64];
        self.rand_gen.fill(&mut secret)?;
        let session_service = SessionLayer::new(MemoryStore::new(), &secret);

        let rp_origin = Url::parse(&format!("https://{}", setup.application_domain))?;
        let webauthn = WebauthnBuilder::new(&setup.application_domain, &rp_origin)?.build()?;
        let app_serv = self
            .create_router(session_service, Arc::new(webauthn))
            .into_make_service();
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

    fn create_router(
        &self,
        session_service: SessionLayer<MemoryStore>,
        webauthn: Arc<Webauthn>,
    ) -> Router {
        let mut app = Router::new().fallback(fallback.into_service());

        app = app.merge(authentication::router(
            self.pool.clone(),
            session_service,
            webauthn,
        ));
        app = app.merge(clients::router(self.pool.clone()));
        app = app.merge(client_groups::router(self.pool.clone()));
        app = app.merge(domains::router(self.pool.clone()));
        app = app.merge(domain_groups::router(self.pool.clone()));
        app = app.merge(groups_applied::router(self.pool.clone()));
        app = app.merge(health::router());
        app = app.merge(setup::router(self.pool.clone()));

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
}

#[derive(Debug, Error)]
pub enum EndpointsError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Recv(#[from] RecvError),
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    #[error(transparent)]
    Rng(#[from] Unspecified),
    #[error(transparent)]
    Url(#[from] ParseError),
    #[error(transparent)]
    Webauthn(#[from] WebauthnError),
}

async fn fallback() -> Redirect {
    Redirect::to("/")
}
