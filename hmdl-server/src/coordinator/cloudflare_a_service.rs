use std::{collections::HashSet, io, net::IpAddr};
use thiserror::Error;
use tokio::{
    net::lookup_host,
    runtime::Handle,
    sync::broadcast::{error::RecvError, Receiver},
    task::{self, JoinError},
};

use crate::certificate::{CloudflareClient, CloudflareClientError};

use super::{HmdlSetup, SetupStatus};

pub struct CloudflareAService {}

impl CloudflareAService {
    pub fn create() -> Self {
        Self {}
    }

    pub async fn start(
        &self,
        mut ip_changed: Receiver<HashSet<IpAddr>>,
        mut install_stat_reciever: Receiver<SetupStatus>,
    ) -> Result<(), CloudflareAServiceError> {
        let (initial_ips, install_status) =
            tokio::join!(ip_changed.recv(), install_stat_reciever.recv());

        let mut ips: HashSet<IpAddr> = initial_ips?
            .into_iter()
            .filter(|x| !x.is_loopback())
            .collect();

        let mut status = install_status?;

        loop {
            if let SetupStatus::InProgress(settings) = &status {
                Self::update_ips(settings, &ips).await?;
            } else if let SetupStatus::Setup(settings) = &status {
                Self::update_ips(settings, &ips).await?;
            }
            tokio::select!(
                Ok(new_ips) = ip_changed.recv() => {
                    tracing::debug!("Got new IPs");
                    ips = new_ips;
                }
                new_install_stat = install_stat_reciever.recv() => {
                    tracing::debug!("Got new install status");
                    status = new_install_stat?;
                }
            );
        }
    }

    async fn update_ips(
        settings: &HmdlSetup,
        ips: &HashSet<IpAddr>,
    ) -> Result<(), CloudflareAServiceError> {
        tracing::debug!("Have Setup, checking cloudflare for ips: {:#?}", ips);

        let api_token = settings.cloudflare_api_token.clone();
        let domain = settings.application_domain.clone();
        let ip_updates = ips.clone();

        let handle = Handle::current();
        let join = handle.spawn_blocking(move || -> Result<(), CloudflareClientError> {
            let cloud_client = CloudflareClient::create(api_token, &domain)?;
            cloud_client.update_dns(ip_updates)?;
            Ok(())
        });

        join.await??;

        Ok(())
    }
}
#[derive(Debug, Error)]
pub enum CloudflareAServiceError {
    #[error(transparent)]
    CloudflareClient(#[from] CloudflareClientError),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Join(#[from] JoinError),
    #[error(transparent)]
    Recv(#[from] RecvError),
}
