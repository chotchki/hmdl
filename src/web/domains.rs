use super::ApiResult;
use axum::{routing::get, Extension, Json, Router};
use serde::Serialize;
use sqlx::{query, query_as, SqlitePool};

use super::ApiContext;

pub fn router(pool: SqlitePool) -> Router {
    Router::new()
        .route("/api/domains", get(list_domains))
        .layer(Extension(ApiContext { pool }))
}

#[derive(Serialize)]
struct Domain {
    name: String,
}

async fn list_domains(ctx: Extension<ApiContext>) -> ApiResult<Json<Vec<Domain>>> {
    let mut conn = ctx.pool.acquire().await?;

    let domains = query_as!(
        Domain,
        r#"
        SELECT name
        FROM known_domains
        ORDER BY name
        "#
    )
    .fetch_all(&mut conn)
    .await?;

    Ok(Json(domains))
}
