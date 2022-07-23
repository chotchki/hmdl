use crate::web::util::{ApiContext, ApiResult};

use axum::{routing::post, Extension, Json, Router};
use serde::Deserialize;
use sqlx::{query, SqlitePool};

pub fn router(pool: SqlitePool) -> Router {
    Router::new()
        .route(
            "/api/groups-applied",
            post(add_domain_to_client).put(del_domain_from_client),
        )
        .layer(Extension(ApiContext { pool }))
}

#[derive(Deserialize)]
struct DomainClient {
    client_group: String,
    domain_group: String,
}

async fn add_domain_to_client(
    ctx: Extension<ApiContext>,
    Json(req): Json<DomainClient>,
) -> ApiResult<Json<()>> {
    let mut conn = ctx.pool.acquire().await?;

    query!(
        r#"
        INSERT INTO groups_applied (client_group_name, domain_group_name) VALUES (?1, ?2)
        "#,
        req.client_group,
        req.domain_group
    )
    .execute(&mut conn)
    .await?;

    Ok(Json(()))
}

async fn del_domain_from_client(
    ctx: Extension<ApiContext>,
    Json(req): Json<DomainClient>,
) -> ApiResult<Json<()>> {
    let mut conn = ctx.pool.acquire().await?;

    query!(
        r#"
        DELETE FROM groups_applied 
        WHERE
            client_group_name = ?1
            and domain_group_name = ?2
        "#,
        req.client_group,
        req.domain_group
    )
    .execute(&mut conn)
    .await?;

    Ok(Json(()))
}
