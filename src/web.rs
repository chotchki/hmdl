mod admin_server;
pub use admin_server::AdminServer;

mod api_context;
pub use api_context::ApiContext;

pub mod domains;

mod api_error;
pub use api_error::ApiError;

pub type ApiResult<T, E = ApiError> = std::result::Result<T, E>;
