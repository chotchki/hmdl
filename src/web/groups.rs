use super::{domains::Domain, ApiResult};
use axum::{
    extract::Path,
    routing::{delete, get},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, SqlitePool};

use super::ApiContext;

pub fn router(pool: SqlitePool) -> Router {
    Router::new()
        .route("/api/groups", get(list_groups))
        .route(
            "/api/group/:name",
            delete(delete_group).post(add_group).put(update_group),
        )
        .route("/api/group/:name/domains", get(list_group_domains))
        .route(
            "/api/group/:name/domains/:domain_name",
            delete(delete_domain_from_group),
        )
        .layer(Extension(ApiContext { pool }))
}

#[derive(Deserialize, Serialize, sqlx::FromRow)]
struct Group {
    name: String,
    model_status: String,
}

async fn add_group(ctx: Extension<ApiContext>, Path(name): Path<String>) -> ApiResult<Json<()>> {
    let mut conn = ctx.pool.acquire().await?;

    query!(
        r#"
        INSERT INTO groups (name) VALUES (?1)
        "#,
        name
    )
    .execute(&mut conn)
    .await?;

    Ok(Json(()))
}

async fn delete_group(ctx: Extension<ApiContext>, Path(name): Path<String>) -> ApiResult<Json<()>> {
    let mut conn = ctx.pool.acquire().await?;

    query!(
        r#"
        DELETE FROM groups where name = ?1
        "#,
        name
    )
    .execute(&mut conn)
    .await?;

    Ok(Json(()))
}

async fn delete_domain_from_group(
    ctx: Extension<ApiContext>,
    Path((name, domain_name)): Path<(String, String)>,
) -> ApiResult<Json<()>> {
    let mut conn = ctx.pool.acquire().await?;

    query!(
        r#"
        DELETE FROM domain_group_member where domain_name = ?1 and group_name = ?2
        "#,
        domain_name,
        name
    )
    .execute(&mut conn)
    .await?;

    Ok(Json(()))
}

async fn list_groups(ctx: Extension<ApiContext>) -> ApiResult<Json<Vec<Group>>> {
    let mut conn = ctx.pool.acquire().await?;

    let groups = query_as!(
        Group,
        r#"
        SELECT name, model_status
        FROM groups
        ORDER BY name
        "#
    )
    .fetch_all(&mut conn)
    .await?;

    Ok(Json(groups))
}

async fn list_group_domains(
    ctx: Extension<ApiContext>,
    Path(name): Path<String>,
) -> ApiResult<Json<Vec<Domain>>> {
    let mut conn = ctx.pool.acquire().await?;

    let domains = query_as!(
        Domain,
        r#"
        SELECT name, last_seen, last_client
        FROM known_domains
            INNER JOIN domain_group_member on domain_group_member.domain_name = known_domains.name
        WHERE group_name = ?1
        ORDER BY name
        "#,
        name
    )
    .fetch_all(&mut conn)
    .await?;

    Ok(Json(domains))
}

async fn update_group(
    ctx: Extension<ApiContext>,
    Path(name): Path<String>,
    Json(req): Json<Group>,
) -> ApiResult<Json<()>> {
    let mut conn = ctx.pool.acquire().await?;

    query!(
        r#"
        UPDATE groups
        SET name = ?1,
            model_status = ?2
        WHERE name = ?3
        "#,
        req.name,
        req.model_status,
        name
    )
    .execute(&mut conn)
    .await?;

    Ok(Json(()))
}
