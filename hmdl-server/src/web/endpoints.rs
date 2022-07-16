pub mod client_groups;
pub mod clients;
pub mod domain_groups;
pub mod domains;
pub mod health;

//Only propogate frontend if in release mode
#[cfg(not(debug_assertions))]
pub mod frontend;
