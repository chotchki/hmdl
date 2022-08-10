mod api_context;
pub use api_context::ApiContext;
pub use api_context::ApiContextSetup;

mod api_error;
pub use api_error::ApiError;

pub type ApiResult<T, E = ApiError> = std::result::Result<T, E>;
