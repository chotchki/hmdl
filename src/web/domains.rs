use std::net::SocketAddr;

use super::ApiResult;
use axum::{routing::get, Extension, Json, Router};
use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
use serde::Serialize;
use sqlx::{query, query_as, sqlite::SqliteRow, FromRow, Sqlite, SqlitePool};

use super::ApiContext;

pub fn router(pool: SqlitePool) -> Router {
    Router::new()
        .route("/api/domains", get(list_domains))
        .layer(Extension(ApiContext { pool }))
}

#[derive(Serialize, sqlx::FromRow)]
struct Domain {
    name: String,
    last_seen: NaiveDateTime,
    last_client: String,
}

async fn list_domains(ctx: Extension<ApiContext>) -> ApiResult<Json<Vec<Domain>>> {
    let mut conn = ctx.pool.acquire().await?;

    let domains = query_as!(
        Domain,
        r#"
        SELECT name, last_seen, last_client
        FROM known_domains
        ORDER BY name
        "#
    )
    .fetch_all(&mut conn)
    .await?;

    Ok(Json(domains))
}
/*
try_map(|row: SqliteRow| {
        Ok(Domain {
            name: row.index(0),
            last_seen: DateTime::parse_from_str(row.index(1), "%+")?,
            last_client: SocketAddr::try_from(row.index(2))?,
        })
    }) */
