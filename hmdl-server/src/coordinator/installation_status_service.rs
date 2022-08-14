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
    }

    async fn setup_status_db_check(
        conn: &mut SqliteConnection,
    ) -> Result<SetupStatus, InstallationStatusServiceError> {
        let setting_record = query!(
            r#"
            SELECT application_domain, cloudflare_api_token, acme_email, https_started_once
            FROM hmdl_settings
            WHERE lock_column == true
            "#
        )
        .fetch_optional(conn)
        .await?;

        if let Some(rec) = setting_record {
            let settings = HmdlSetup {
                application_domain: rec.application_domain,
                cloudflare_api_token: rec.cloudflare_api_token,
                acme_email: rec.acme_email,
            };

            if rec.https_started_once {
                Ok(SetupStatus::Setup(settings))
            } else {
                Ok(SetupStatus::InProgress(settings))
            }
        } else {
            Ok(SetupStatus::NotSetup)
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SetupStatus {
    NotSetup,              //Should show the setup screen
    InProgress(HmdlSetup), //This is pending the HTTPS server being up, how do I tell this is the state?! Maybe another signal?
    Setup(HmdlSetup),      //The HTTPS server is up and everything should switch over to it
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
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
