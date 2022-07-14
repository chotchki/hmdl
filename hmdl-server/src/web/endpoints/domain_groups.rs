use crate::web::util::{ApiContext, ApiResult};

use axum::{extract::Path, routing::get, Extension, Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::{query, SqlitePool};

pub fn router(pool: SqlitePool) -> Router {
    Router::new()
        .route("/api/domain-groups", get(list_groups))
        .route(
            "/api/domain-groups/:name",
            get(list_group_detail)
                .delete(delete_group)
                .post(add_group)
                .put(update_group),
        )
        .layer(Extension(ApiContext { pool }))
}

async fn list_groups(ctx: Extension<ApiContext>) -> ApiResult<Json<Vec<String>>> {
    let mut conn = ctx.pool.acquire().await?;

    let groups = query!(
        r#"
        SELECT name
        FROM domain_groups
        ORDER BY name
        "#
    )
    .map(|x| x.name)
    .fetch_all(&mut conn)
    .await?;

    Ok(Json(groups))
}

#[derive(Deserialize, Serialize, sqlx::FromRow)]
struct GroupDetail {
    name: String,
    model_status: String,
    domains: Vec<String>,
}

async fn list_group_detail(
    ctx: Extension<ApiContext>,
    Path(name): Path<String>,
) -> ApiResult<Json<GroupDetail>> {
    let mut conn = ctx.pool.acquire().await?;

    let (group_name, model_status) = query!(
        r#"
        SELECT name, model_status
        FROM domain_groups
        ORDER BY name
        "#
    )
    .map(|x| (x.name, x.model_status))
    .fetch_one(&mut conn)
    .await?;

    let domains = query!(
        r#"
        SELECT domain_name
        FROM domain_group_member
        WHERE group_name = ?1
        ORDER BY domain_name
        "#,
        name
    )
    .map(|x| x.domain_name)
    .fetch_all(&mut conn)
    .await?;

    Ok(Json(GroupDetail {
        name: group_name,
        model_status,
        domains,
    }))
}

async fn add_group(ctx: Extension<ApiContext>, Path(name): Path<String>) -> ApiResult<Json<()>> {
    let mut conn = ctx.pool.acquire().await?;

    query!(
        r#"
        INSERT INTO domain_groups (name) VALUES (?1)
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
        DELETE FROM domain_groups where name = ?1
        "#,
        name
    )
    .execute(&mut conn)
    .await?;

    Ok(Json(()))
}

#[derive(Deserialize)]
struct UpdateGroup {
    name: String,
    model_status: String,
}

async fn update_group(
    ctx: Extension<ApiContext>,
    Path(name): Path<String>,
    Json(req): Json<UpdateGroup>,
) -> ApiResult<Json<()>> {
    let mut conn = ctx.pool.acquire().await?;

    query!(
        r#"
        UPDATE domain_groups
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
