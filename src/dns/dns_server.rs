use std::{
    io,
    net::{IpAddr, Ipv4Addr, SocketAddr, ToSocketAddrs},
    sync::Arc,
    time::Duration,
};

use tokio::{
    net::{TcpListener, TcpSocket, UdpSocket},
    runtime::Runtime,
    sync::Mutex,
};
use trust_dns_server::{authority::Catalog, ServerFuture};

/// This is an extremely opinionated forwarding DNS server used for agressive filtering
pub struct DnsServer;

//Hint on existing DNS server example: https://github.com/bluejekyll/trust-dns/blob/main/bin/src/named.rs

const PORT: u16 = 53;
const TIMEOUT: Duration = Duration::new(30, 0);

impl DnsServer {
    pub async fn start_dns() -> io::Result<()> {
        let catalog: Catalog = Catalog::new();
        let mut server = ServerFuture::new(catalog);

        let listen_addr: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), PORT);

        let udp_socket = UdpSocket::bind(listen_addr).await?;
        server.register_socket(udp_socket);

        let tcp_listener = TcpListener::bind(listen_addr).await?;
        server.register_listener(tcp_listener, TIMEOUT);

        server.block_until_done().await?;

        Ok(())
    }
}
