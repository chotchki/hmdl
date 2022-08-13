use std::{collections::HashSet, io, net::IpAddr};
use thiserror::Error;
use tokio::{
    net::lookup_host,
    sync::broadcast::{error::RecvError, Receiver},
    task::{self, JoinError},
};

use crate::certificate::{CloudflareClient, CloudflareClientError};

use super::SetupStatus;

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
            if let SetupStatus::Setup(settings) = &status {
                tracing::debug!("Have Setup, checking cloudflare for ips: {:#?}", ips);
                //Now check if DNS is already correct, don't want to call cloudflare unneccessarily
                let resolved_ips: HashSet<IpAddr> =
                    lookup_host(settings.application_domain.clone())
                        .await?
                        .map(|x| x.ip())
                        .collect();

                if ips == resolved_ips {
                    tracing::debug!("DNS Settings match, no cloudflare update")
                } else {
                    //Now we need to figure out the sets of actions to take
                    let cloud_client = CloudflareClient::create(
                        settings.cloudflare_api_token.clone(),
                        &settings.application_domain,
                    )?;

                    let ip_updates = ips.clone();
                    task::spawn_blocking(move || cloud_client.update_dns(ip_updates)).await??;
                }
            } else {
                tokio::select!(
                    Ok(new_ips) = ip_changed.recv() => {
                        tracing::debug!("Got new IPs");
                        ips = new_ips;
                    }
                    new_install_stat = install_stat_reciever.recv() => {
                        tracing::debug!("Got new install status");
                        status = new_install_stat?;
                    }
                )
            }
        }
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
