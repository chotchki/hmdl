use std::sync::Arc;

use acme_lib::{create_p256_key, Certificate, Directory, DirectoryUrl};
use sqlx::SqlitePool;
use thiserror::Error;
use tokio::runtime::Handle;

use super::{cloudflare_setup::CloudflareSetupError, AcmePersistKey, CloudflareSetup};

pub struct CertManager<'a> {
    acme_email: String,
    cloud: CloudflareSetup,
    domain: String,
    persist: AcmePersistKey,
    url: DirectoryUrl<'a>,
}

impl<'a> CertManager<'a> {
    pub fn create(
        pool: SqlitePool,
        handle: Arc<Handle>,
        domain: String,
        api_token: String,
        acme_email: String,
    ) -> Result<Self, CertManagerError> {
        let url = if cfg!(debug_assertions) {
            DirectoryUrl::LetsEncryptStaging
        } else {
            DirectoryUrl::LetsEncrypt
        };

        Ok(Self {
            acme_email,
            cloud: CloudflareSetup::create(api_token, &domain)?,
            domain,
            persist: AcmePersistKey::create(pool, handle),
            url,
        })
    }

    pub fn set_dns_get_cert(&self) -> Result<Certificate, CertManagerError> {
        //Start by making sure our domain dns is reasonable up to date
        self.cloud.update_dns()?;

        let dir = Directory::from_url(self.persist.clone(), self.url.clone())?;
        let acc = dir.account(&self.acme_email)?;

        let maybe_cert = acc.certificate(&self.domain)?;

        if let Some(cert) = maybe_cert {
            if cert.valid_days_left() > 30 {
                return Ok(cert);
            }
        }

        let mut ord_new = acc.new_order(&self.domain, &[])?;

        let ord_csr = loop {
            if let Some(ord_csr) = ord_new.confirm_validations() {
                break ord_csr;
            }

            let auths = ord_new.authorizations()?;
            let chall = auths[0].dns_challenge();

            self.cloud.create_proof(chall.dns_proof())?;

            chall.validate(5000)?;

            ord_new.refresh()?;
        };

        let pkey_pri = create_p256_key();

        let ord_cert = ord_csr.finalize_pkey(pkey_pri, 5000)?;

        let cert = ord_cert.download_and_save_cert()?;

        Ok(cert)
    }
}

#[derive(Debug, Error)]
pub enum CertManagerError {
    #[error(transparent)]
    AcmeError(#[from] acme_lib::Error),

    #[error(transparent)]
    CloudflareSetupError(#[from] CloudflareSetupError),
}
