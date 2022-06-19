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
use trust_dns_server::store::forwarder::ForwardAuthority;
use trust_dns_server::{
    authority::{AuthorityObject, Catalog, ZoneType},
    client::rr::Name,
    resolver::config::{NameServerConfigGroup, ResolverOpts},
    store::forwarder::ForwardConfig,
    ServerFuture,
};

/// This is an extremely opinionated forwarding DNS server used for agressive filtering
pub struct DnsServer;

//Hint on existing DNS server example: https://github.com/bluejekyll/trust-dns/blob/main/bin/src/named.rs

const PORT: u16 = 53;
const TIMEOUT: Duration = Duration::new(30, 0);

impl DnsServer {
    pub async fn create() -> io::Result<()> {
        let mut catalog: Catalog = Catalog::new();

        //let fa = ForwardAuthority::new(TokioHandle).await.unwrap(); //TODO I don't like this
        let fa_config = ForwardConfig {
            name_servers: NameServerConfigGroup::google(),
            options: Some(ResolverOpts::default()),
        };
        let fa = ForwardAuthority::try_from_config(Name::root(), ZoneType::Forward, &fa_config)
            .await
            .unwrap();

        catalog.upsert(
            Name::root().into(),
            Box::new(Arc::new(fa)) as Box<dyn AuthorityObject>,
        );
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
