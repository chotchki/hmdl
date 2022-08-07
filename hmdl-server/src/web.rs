mod admin_server;
pub use admin_server::AdminServer;

pub mod endpoints;

//Only propogate frontend if in release mode
#[cfg(not(debug_assertions))]
pub mod frontend;

pub mod install_endpoints;
pub mod util;
