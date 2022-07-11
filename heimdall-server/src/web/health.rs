use super::ApiResult;
use axum::{routing::get, Json, Router};

pub fn router() -> Router {
    Router::new().route("/api/health", get(health))
}

async fn health() -> ApiResult<Json<String>> {
    Ok(Json("Ok".to_string()))
}
