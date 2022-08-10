use serde::Deserialize;
use sqlx::{query, SqliteConnection, SqlitePool};
use thiserror::Error;
use tokio::sync::broadcast::{error::SendError, Receiver, Sender};

pub struct InstallationStatusService {
    pool: SqlitePool,
}

impl InstallationStatusService {
    pub fn create(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn start(
        &self,
        mut request_refresh: Receiver<()>,
        installation_status: Sender<SetupStatus>,
    ) -> Result<(), InstallationStatusServiceError> {
        tracing::debug!("Checking installation status");
        let mut conn = self.pool.acquire().await?;

        installation_status.send(Self::setup_status_db_check(&mut conn).await?)?;

        loop {
            tokio::select! {
                Ok(()) = request_refresh.recv() => {
                    tracing::info!("Requested refresh of setup status.");
                    installation_status.send(Self::setup_status_db_check(&mut conn).await?)?;
                }
            }
        }

        Ok(())
    }

    async fn setup_status_db_check(
        conn: &mut SqliteConnection,
    ) -> Result<SetupStatus, InstallationStatusServiceError> {
        let setting_record = query!(
            r#"
            SELECT application_domain, cloudflare_api_token, acme_email
            FROM hmdl_settings
            WHERE lock_column == true
            "#
        )
        .fetch_optional(conn)
        .await?;

        if let Some(rec) = setting_record {
            Ok(SetupStatus::Setup(HmdlSetup {
                application_domain: rec.application_domain,
                cloudflare_api_token: rec.cloudflare_api_token,
                acme_email: rec.acme_email,
            }))
        } else {
            Ok(SetupStatus::NotSetup)
        }
    }
}

#[derive(Clone, Debug)]
pub enum SetupStatus {
    Setup(HmdlSetup),
    NotSetup,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HmdlSetup {
    pub application_domain: String,
    pub cloudflare_api_token: String,
    pub acme_email: String,
}

#[derive(Debug, Error)]
pub enum InstallationStatusServiceError {
    #[error(transparent)]
    Send(#[from] SendError<SetupStatus>),

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}
