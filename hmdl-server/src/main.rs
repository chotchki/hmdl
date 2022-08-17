pub mod certificate;
pub mod coordinator;
pub mod dns;
pub mod web;

use std::env;

use git_version::git_version;

use crate::coordinator::Coordinator;
pub const GIT_VERSION: &str = git_version!();

#[tokio::main]
async fn main() {
    // initialize tracing/logging
    console_subscriber::init();
    tracing::warn!("Starting hmdl version {}", GIT_VERSION);

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        tracing::error!("Database path not provided, exiting");
        return;
    }

    let path = args.get(1).unwrap();
    tracing::debug!("Database path is {}", path);

    let mut coordinator = Coordinator::create(path)
        .await
        .expect("Unable to create the HMDL coordinator");

    coordinator.start().await.expect("The coordinator exited");
}
