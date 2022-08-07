use super::FilteringForwarder;
use crate::coordinator::IpProvderServiceError;
use sqlx::SqlitePool;
use std::{
    collections::HashSet,
    io,
    net::{IpAddr, Ipv6Addr, SocketAddr},
    sync::Arc,
    time::Duration,
};
use thiserror::Error;
use tokio::{
    net::{TcpListener, UdpSocket},
    sync::broadcast::{error::RecvError, Receiver},
};
use trust_dns_server::{
    authority::{AuthorityObject, Catalog},
    client::rr::Name,
    ServerFuture,
};

const PORT: u16 = 53;
const TIMEOUT: Duration = Duration::new(30, 0);

/// This is an extremely opinionated forwarding DNS server used for agressive filtering
/// Hint on existing DNS server example: https://github.com/bluejekyll/trust-dns/blob/main/bin/src/named.rs
pub struct DnsServer {
    filtering_forwarder: Arc<FilteringForwarder>,
}

impl DnsServer {
    pub async fn create(pool: SqlitePool) -> Self {
        let filtering_forwarder = Arc::new(FilteringForwarder::create(pool.clone()).await);
        Self {
            filtering_forwarder,
        }
    }

    pub async fn start(
        &self,
        mut ip_changed: Receiver<HashSet<IpAddr>>,
    ) -> Result<(), DnsServerError> {
        let mut ips = ip_changed.recv().await?;

        loop {
            let mut catalog: Catalog = Catalog::new();

            catalog.upsert(
                Name::root().into(),
                Box::new(self.filtering_forwarder.clone()) as Box<dyn AuthorityObject>,
            );
            let mut server = ServerFuture::new(catalog);

            let listen_addr = SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), PORT);

            let udp_socket = UdpSocket::bind(listen_addr).await?;
            server.register_socket(udp_socket);

            let tcp_listener = TcpListener::bind(listen_addr).await?;
            server.register_listener(tcp_listener, TIMEOUT);

            tokio::select! {
                Ok(()) = server.block_until_done() => {
                },
                Ok(new_ip) = ip_changed.recv() => {
                    tracing::info!("Recieved IP change, restarting DNS server");
                    ips = new_ip;
                }, else => {
                    tracing::warn!("Futures aborted, shutting down DNS");
                }
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum DnsServerError {
    #[error(transparent)]
    IoResult(#[from] io::Error),

    #[error(transparent)]
    IpProvderService(#[from] IpProvderServiceError),

    #[error(transparent)]
    Recv(#[from] RecvError),
}
