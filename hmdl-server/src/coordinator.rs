use hmdl_db::DatabaseHandle;
use sqlx::SqlitePool;
use thiserror::Error;
use tokio::sync::broadcast::{self};
use tokio::task::JoinError;

/// The goal of the coordinator is to start up the various listening servers of HMDL
/// and allow their requested restart/reload as needed. This stems from the need to
/// start the HMDL server insecurely and then once a HTTS certificate is found / renewed
/// to bring that up too.
///
/// I'm changing course to instead use message pasisng to try and simplify all of this
mod cloudflare_a_service;
use cloudflare_a_service::CloudflareAService;

mod installation_status_service;
pub use installation_status_service::HmdlSetup;
use installation_status_service::InstallationStatusService;
pub use installation_status_service::SetupStatus;

mod ip_provider_service;
use crate::dns::DnsServer;
use crate::web::install_endpoints::InstallEndpoints;

pub use self::ip_provider_service::{IpProvderService, IpProvderServiceError};

pub struct Coordinator {
    pool: SqlitePool,
    installation_status_service: InstallationStatusService,
    ip_provider_service: IpProvderService,
    dns_server_service: DnsServer,
    install_endpoints: InstallEndpoints,
    cloudflare_a_service: CloudflareAService,
}

impl Coordinator {
    pub async fn create() -> Result<Self, CoordinatorError> {
        let pool = DatabaseHandle::create().await?;

        let installation_status_service = InstallationStatusService::create(pool.clone());
        let ip_provider_service = IpProvderService::create();
        let dns_server_service = DnsServer::create(pool.clone()).await;
        let install_endpoints = InstallEndpoints::create(pool.clone());
        let cloudflare_a_service = CloudflareAService::create();

        Ok(Self {
            pool,
            installation_status_service,
            ip_provider_service,
            dns_server_service,
            install_endpoints,
            cloudflare_a_service,
        })
    }

    pub async fn start(&mut self) -> Result<(), CoordinatorError> {
        let (install_refresh_sender, install_refresh_reciever) = broadcast::channel(1);
        let (install_stat_sender, install_stat_reciever) = broadcast::channel(1);
        let install_stat_reciever2 = install_stat_sender.subscribe();
        let (ip_provider_sender, ip_provider_reciever) = broadcast::channel(1);
        let ip_provider_reciever2 = ip_provider_sender.subscribe();

        //let (https_ready_sender, https_ready_reciever) = broadcast::channel(1);

        tokio::select! {
            Ok(()) = self.installation_status_service.start(install_refresh_reciever, install_stat_sender) => {
                tracing::debug!("Install Status Service exited.");
            }
            Ok(()) = self.ip_provider_service.start(ip_provider_sender) => {
                tracing::debug!("IP Provider exited.");
            }
            Ok(()) = self.dns_server_service.start(ip_provider_reciever) => {
                tracing::debug!("DNS Server exited.");
            }
            Ok(()) = self.install_endpoints.start(install_stat_reciever, install_refresh_sender) => {
                tracing::debug!("Install Endpoints exited.");
            }
            r = self.cloudflare_a_service.start(ip_provider_reciever2, install_stat_reciever2) => {
                match r {
                    Ok(()) => tracing::debug!("Cloudflare A/AAAA record service exited."),
                    Err(e) => tracing::error!("Cloudflare A/AAAA had an error {}", e)
                }
            }
            /*Ok(()) = self.cloudflare_proof_service.start(cloudflare_proof_reciever, acme_refresh_sender) => {
                tracing::debug!("Cloudflare proof service exited.");
            }
            Ok(()) = self.acme_provision_service.start(install_stat_reciever, cloudflare_proof_sender, acme_refresh_reciever, cert_sender) => {
                tracing::debug!("Acme Service exited.");
            }
            Ok(()) = self.admin_server.start(install_stat_reciever, cert_reciever) => {
                tracing::debug!("Admin Endpoints Exited.");
            }*/
        }

        /// 1. Start a service to check for settings
        /// So DNS should always start
        ///     listen on any address
        ///     TODO: Limit responses to internal networks, maybe pass to the forwarding authority?
        ///
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
