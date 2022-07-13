pub mod db;
pub mod dns;
pub mod web;

use hmdl_db::DatabaseHandle;
use tracing_subscriber::{
    fmt, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

use git_version::git_version;

use crate::{dns::DnsServer, web::AdminServer};
pub const GIT_VERSION: &str = git_version!();

#[tokio::main]
async fn main() {
    // initialize tracing/logging
    let fmt_layer = fmt::layer().with_target(false);
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();

    tracing::info!("Starting HearthStone version {}", GIT_VERSION);

    let pool = DatabaseHandle::create().await.unwrap();

    let mut handles = vec![];
    let as_pool = pool.clone();
    handles.push(tokio::spawn(async move {
        AdminServer::create(as_pool).await.unwrap();
    }));

    let dns_pool = pool.clone();
    handles.push(tokio::spawn(async move {
        tracing::info!("Starting DNS Server");
        DnsServer::create(dns_pool).await.unwrap();
    }));

    futures::future::join_all(handles).await;
}
