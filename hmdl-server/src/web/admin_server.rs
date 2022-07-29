use crate::certificate::{CertManager, CertManagerError};
use crate::web::endpoints::{
    client_groups, clients, domain_groups, domains, groups_applied, health, setup,
};
use acme_lib::Certificate;
use axum::{handler::Handler, http::StatusCode, Router};
use axum_server::tls_rustls::RustlsConfig;
use rustls::ServerConfig;
use sqlx::{query, SqlitePool};
use std::{net::SocketAddr, sync::Arc};
use thiserror::Error;
use tokio::runtime::Handle;
use tokio::sync::mpsc::unbounded_channel;
use tokio::task::{JoinError, JoinHandle};

pub struct AdminServer {
    pool: SqlitePool,
}

impl AdminServer {
    pub fn create(pool: SqlitePool) -> AdminServer {
        let (sender, recv) = unbounded_channel::<()>();
        AdminServer { pool }
    }

    fn create_router(pool: SqlitePool) -> Router {
        let mut app = Router::new().fallback(fallback.into_service());

        app = app.merge(clients::router(pool.clone()));
        app = app.merge(client_groups::router(pool.clone()));
        app = app.merge(domains::router(pool.clone()));
        app = app.merge(domain_groups::router(pool.clone()));
        app = app.merge(groups_applied::router(pool.clone()));
        app = app.merge(health::router());
        app = app.merge(setup::router(pool));

        //Only enable static content if we're in release mode
        #[cfg(not(debug_assertions))]
        {
            app = app.merge(crate::web::endpoints::frontend::router());
        }

        app
    }

    pub async fn start(&self) -> Result<Vec<JoinHandle<()>>, AdminServerError> {
        let mut handles = vec![];

        let pool = self.pool.clone();

        let addr = SocketAddr::from(([0, 0, 0, 0], 80));
        let app_serv = Self::create_router(pool).into_make_service();
        let builder = axum_server::bind(addr);
        tracing::info!("HTTP Server listening on {}", addr);

        handles.push(tokio::spawn(async move {
            builder.serve(app_serv).await.unwrap();
        }));

        let pool = self.pool.clone();
        let maybe_cert = self.get_server_certs().await?;
        if let Some(cert) = maybe_cert {
            let rusttls_cfg = RustlsConfig::from_config(self.get_rusttls_config(cert)?);

            let addr = SocketAddr::from(([0, 0, 0, 0], 443));
            let app_serv = Self::create_router(pool).into_make_service();
            let builder = axum_server::bind_rustls(addr, rusttls_cfg);
            tracing::info!("HTTPS Server listening on {}", addr);

            handles.push(tokio::spawn(async move {
                builder.serve(app_serv).await.unwrap();
            }));
        }

        Ok(handles)
    }

    fn get_rusttls_config(
        &self,
        acme_cert: acme_lib::Certificate,
    ) -> Result<Arc<ServerConfig>, AdminServerError> {
        let rustls_certs = vec![rustls::Certificate(acme_cert.certificate_der())];
        let rustls_private_key = rustls::PrivateKey(acme_cert.private_key_der());

        Ok(Arc::new(
            ServerConfig::builder()
                .with_safe_defaults()
                .with_no_client_auth()
                .with_single_cert(rustls_certs, rustls_private_key)?,
        ))
    }

    async fn get_server_certs(&self) -> Result<Option<Certificate>, AdminServerError> {
        let mut conn = self.pool.acquire().await?;

        let setting_record = query!(
            r#"
            SELECT application_domain, cloudflare_api_token, acme_email
            FROM hmdl_settings
            WHERE lock_column == true
            "#
        )
        .fetch_optional(&mut conn)
        .await?;

        let pool = self.pool.clone();
        if let Some(s) = setting_record {
            let join_hdl = Handle::current().spawn_blocking(move || {
                Self::get_cert_and_update_dns(
                    pool,
                    Handle::current(),
                    s.application_domain,
                    s.cloudflare_api_token,
                    s.acme_email,
                )
            });
            Ok(Some(join_hdl.await??))
        } else {
            Ok(None)
        }
    }

    fn get_cert_and_update_dns(
        pool: SqlitePool,
        handle: Handle,
        domain: String,
        api_token: String,
        acme_email: String,
    ) -> Result<Certificate, AdminServerError> {
        let cert_man = CertManager::create(pool, Arc::new(handle), domain, api_token, acme_email)?;

        Ok(cert_man.set_dns_get_cert()?)
    }
}

#[derive(Debug, Error)]
pub enum AdminServerError {
    #[error(transparent)]
    CertManagerError(#[from] CertManagerError),

    #[error(transparent)]
    HyperError(#[from] hyper::Error),

    #[error(transparent)]
    JoinError(#[from] JoinError),

    #[error("Missing acme email")]
    MissingAcmeEmail,

    #[error(transparent)]
    RustlsError(#[from] rustls::Error),

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
}

async fn fallback() -> (StatusCode, String) {
    (
        StatusCode::NOT_FOUND,
        "404 - Yeah you're not finding what you want.".to_string(),
    )
}
