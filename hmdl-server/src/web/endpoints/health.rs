use axum::{routing::get, Json, Router};

use crate::web::util::ApiResult;

pub fn router() -> Router {
    Router::new().route("/api/health", get(health))
}

async fn health() -> ApiResult<Json<String>> {
    Ok(Json("Ok".to_string()))
}
