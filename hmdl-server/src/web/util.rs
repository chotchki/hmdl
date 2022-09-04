mod api_context;
pub use api_context::ApiContext;
pub use api_context::ApiContextAuth;
pub use api_context::ApiContextSetup;

mod api_error;
pub use api_error::ApiError;

mod authorization_check;
pub use authorization_check::is_admin;

pub type ApiResult<T, E = ApiError> = std::result::Result<T, E>;
