use std::collections::HashSet;
use std::net::IpAddr;

use futures::StreamExt;
use futures::{stream::FuturesUnordered, Future};
use hmdl_db::DatabaseHandle;
use sqlx::SqlitePool;
use thiserror::Error;

/// The goal of the coordinator is to start up the various listening servers of HMDL
/// and allow their requested restart/reload as needed. This stems from the need to
/// start the HMDL server insecurely and then once a HTTS certificate is found / renewed
/// to bring that up too.
///
/// I'm changing course to instead use message pasisng to try and simplify all of this
mod hmdl_server_trait;
pub use hmdl_server_trait::{HmdlServerError, HmdlServerTrait};
use tokio::sync::broadcast::{self, Receiver};
use tokio::task::{JoinError, JoinHandle};

mod ip_provider_service;
use crate::dns::DnsServer;

pub use self::ip_provider_service::{IpProvderService, IpProvderServiceError};

pub struct Coordinator {
    pool: SqlitePool,
    ip_provider_service: IpProvderService,
    dns_server_service: DnsServer,
}

impl Coordinator {
    pub async fn create() -> Result<Self, CoordinatorError> {
        let pool = DatabaseHandle::create().await?;

        let ip_provider_service = IpProvderService::create();
        let dns_server_service = DnsServer::create(pool.clone()).await;

        Ok(Self {
            pool,
            ip_provider_service,
            dns_server_service,
        })
    }

    pub async fn start(&mut self) -> Result<(), CoordinatorError> {
        let (ip_provider_sender, ip_provider_reciever) = broadcast::channel(1);
        //let (dns_server_sender, dns_server_reciever) = broadcast::channel(1);

        tokio::select! {
            Ok(()) = self.ip_provider_service.start(ip_provider_sender) => {

            }
            Ok(()) = self.dns_server_service.start(ip_provider_reciever) => {

            }
        }

        /// So DNS should always start
        ///     listen on 127.0.0.1, ::1 and any internal local address
        ///
        /// HTTP should only start on localhost with the minimum install endpoints
        ///     Once installed, HTTP should just provide redirects to HTTPS
        ///
        ///     listen on 127.0.0.1 and ::1
        ///
        /// HTTPS needs to know its domain, the domain be setup and have certificates to start
        ///     In addition it needs to be able refresh its certificates since Let's Encrypt rotates
        ///     them quickly.
        ///
        ///     lsiten on 127.0.0.1, ::1 and any address
        ///
        /// Eventually DHCP will also need to start post setup too.
        ///     listen on 127.0.0.1, ::1 and any internal local address
        ///
        /// Tokio recommends message passing for all of this
        ///
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum CoordinatorError {
    #[error(transparent)]
    IpTrackingError(#[from] IpProvderServiceError),

    #[error(transparent)]
    JoinError(#[from] JoinError),

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
}
