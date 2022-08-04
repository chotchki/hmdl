pub mod certificate;
pub mod coordinator;
pub mod dns;
pub mod web;

use hmdl_db::DatabaseHandle;

use git_version::git_version;

use crate::{coordinator::Coordinator, dns::DnsServer, web::AdminServer};
pub const GIT_VERSION: &str = git_version!();

#[tokio::main]
async fn main() {
    // initialize tracing/logging
    console_subscriber::init();

    tracing::info!("Starting hmdl version {}", GIT_VERSION);

    let mut coordinator = Coordinator::create()
        .await
        .expect("Unable to create the HMDL coordinator");

    coordinator.start().await.expect("The coordinator exited");

    /*
        let mut handles = vec![];
        let as_pool = pool.clone();
        handles.push(tokio::spawn(async move {
            let server = AdminServer::create(as_pool);
            let server_start = server.start();
            server_start.await.unwrap();
        }));

        let dns_pool = pool.clone();
        handles.push(tokio::spawn(async move {
            tracing::info!("Starting DNS Server");
            DnsServer::create(dns_pool).await.unwrap();
        }));


    futures::future::join_all(handles).await;
    */
}
