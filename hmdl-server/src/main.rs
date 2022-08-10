pub mod certificate;
pub mod coordinator;
pub mod dns;
pub mod web;

use git_version::git_version;

use crate::coordinator::Coordinator;
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
}
