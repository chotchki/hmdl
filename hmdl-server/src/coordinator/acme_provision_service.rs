use std::{io, process::Command, string::FromUtf8Error, sync::Arc, time::Duration};

use acme_lib::{create_p256_key, Certificate, Directory, DirectoryUrl};
use axum_server::tls_rustls::RustlsConfig;
use rustls::ServerConfig;
use sqlx::SqlitePool;
use thiserror::Error;
use tokio::{
    runtime::Handle,
    sync::broadcast::{
        error::{RecvError, SendError},
        Receiver, Sender,
    },
    task::JoinError,
};

use crate::certificate::{
    create_proof_domain, AcmePersistKey, CloudflareClient, CloudflareClientError,
};

use super::{HmdlSetup, SetupStatus};

const CERT_REFRESH: Duration = Duration::new(6 * 60 * 60, 0);

pub struct AcmeProvisionService {
    handle: Arc<Handle>,
    persist: AcmePersistKey,
}

impl AcmeProvisionService {
    pub async fn create(pool: SqlitePool) -> Self {
        let handle = Arc::new(Handle::current());
        Self {
            handle: handle.clone(),
            persist: AcmePersistKey::create(pool, handle),
        }
    }

    pub async fn start(
        &self,
        mut install_stat_reciever: Receiver<SetupStatus>,
        tls_config_sender: Sender<(RustlsConfig, HmdlSetup)>,
    ) -> Result<(), AcmeProvisionServiceError> {
        let settings: Result<HmdlSetup, RecvError> = loop {
            let set_val = install_stat_reciever.recv().await?;
            if let SetupStatus::InProgress(s) = set_val {
                break Ok(s);
            } else if let SetupStatus::Setup(s) = set_val {
                break Ok(s);
            }
        };
        let settings = settings?;

        let persist = self.persist.clone();
        let settings2 = settings.clone();
        let acme_cert = self
            .handle
            .spawn_blocking(move || Self::get_certificate(persist, settings2))
            .await??;

        let rustls_certs = vec![rustls::Certificate(acme_cert.certificate_der())];
        let rustls_private_key = rustls::PrivateKey(acme_cert.private_key_der());

        let server_config = Arc::new(
            ServerConfig::builder()
                .with_safe_defaults()
                .with_no_client_auth()
                .with_single_cert(rustls_certs, rustls_private_key)?,
        );
        let rusttls_cfg = RustlsConfig::from_config(server_config);

        tls_config_sender.send((rusttls_cfg.clone(), settings.clone()))?;

        loop {
            tokio::time::sleep(CERT_REFRESH).await;

            let persist = self.persist.clone();
            let settings2 = settings.clone();
            let acme_cert = self
                .handle
                .spawn_blocking(move || Self::get_certificate(persist, settings2))
                .await??;

            let rustls_certs = vec![acme_cert.certificate_der()];

            rusttls_cfg
                .reload_from_der(rustls_certs, acme_cert.private_key_der())
                .await?;
        }
    }

    pub fn get_certificate(
        persist: AcmePersistKey,
        settings: HmdlSetup,
    ) -> Result<Certificate, AcmeProvisionServiceError> {
        let url = DirectoryUrl::LetsEncrypt;

        let cloud_client =
            CloudflareClient::create(settings.cloudflare_api_token, &settings.application_domain)?;

        let dir = Directory::from_url(persist, url)?;
        let acc = dir.account(&settings.acme_email)?;

        let maybe_cert = acc.certificate(&settings.application_domain)?;

        if let Some(cert) = maybe_cert {
            if cert.valid_days_left() > 30 {
                return Ok(cert);
            }
        }

        let mut ord_new = acc.new_order(&settings.application_domain, &[])?;

        let ord_csr = loop {
            if let Some(ord_csr) = ord_new.confirm_validations() {
                break ord_csr;
            }

            let auths = ord_new.authorizations()?;
            let chall = auths[0].dns_challenge();

            cloud_client.create_proof(chall.dns_proof())?;

            //Let's make sure we can see the new proof before we call to refresh ACME
            Self::wait_for_propogation(settings.application_domain.clone(), chall.dns_proof())?;

            chall.validate(1000)?;
            ord_new.refresh()?;
        };

        let pkey_pri = create_p256_key();
        let ord_cert = ord_csr.finalize_pkey(pkey_pri, 5000)?;
        let cert = ord_cert.download_and_save_cert()?;

        Ok(cert)
    }

    fn wait_for_propogation(
        domain: String,
        challenge: String,
    ) -> Result<(), AcmeProvisionServiceError> {
        let domain_proof_str = create_proof_domain(&domain);
        loop {
            let output = Command::new("dig")
                .arg(domain_proof_str.clone())
                .arg("TXT")
                .output()?;
            let output_str = String::from_utf8(output.stdout)?;
            for line in output_str.lines() {
                if line.starts_with(&domain_proof_str) {
                    let line_parts: Vec<&str> = line.split_terminator('"').collect();
                    if let Some(found) = line_parts.get(1) {
                        if *found == challenge {
                            return Ok(());
                        }
                    }
                }
            }

            std::thread::sleep(Duration::from_secs(60));
            tracing::debug!(
                "Domain {} with value {} not found",
                domain_proof_str,
                challenge
            );
        }
    }
}

#[derive(Debug, Error)]
pub enum AcmeProvisionServiceError {
    #[error(transparent)]
    Acme(#[from] acme_lib::Error),
    #[error(transparent)]
    CloudflareClient(#[from] CloudflareClientError),
    #[error(transparent)]
    FromUtf8(#[from] FromUtf8Error),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Join(#[from] JoinError),
    #[error(transparent)]
    Recv(#[from] RecvError),
    #[error(transparent)]
    Rustls(#[from] rustls::Error),
    #[error(transparent)]
    Send(#[from] SendError<(RustlsConfig, HmdlSetup)>),
}
