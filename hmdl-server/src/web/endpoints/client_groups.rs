use crate::web::util::{ApiContext, ApiResult};

use axum::{extract::Path, routing::get, Extension, Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, SqlitePool};

use super::clients::Client;

pub fn router(pool: SqlitePool) -> Router {
    Router::new()
        .route("/api/client-groups", get(list_groups))
        .route(
            "/api/client-groups/:name",
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
        FROM client_groups
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
    clients: Vec<Client>,
    domain_groups: Vec<String>,
}

async fn list_group_detail(
    ctx: Extension<ApiContext>,
    Path(name): Path<String>,
) -> ApiResult<Json<GroupDetail>> {
    let mut conn = ctx.pool.acquire().await?;

    let clients = query_as!(
        Client,
        r#"
        SELECT name, ip, mac
        FROM clients
        INNER JOIN 
        client_group_member ON client_group_member.client_name = clients.name
        WHERE group_name = ?1
        ORDER BY name
        "#,
        name
    )
    .fetch_all(&mut conn)
    .await?;

    let domain_groups = query!(
        r#"
        SELECT domain_group_name
        FROM groups_applied
        WHERE client_group_name = ?1
        "#,
        name
    )
    .map(|x| x.domain_group_name)
    .fetch_all(&mut conn)
    .await?;

    Ok(Json(GroupDetail {
        clients,
        domain_groups,
    }))
}

async fn add_group(ctx: Extension<ApiContext>, Path(name): Path<String>) -> ApiResult<Json<()>> {
    let mut conn = ctx.pool.acquire().await?;

    query!(
        r#"
        INSERT INTO client_groups (name) VALUES (?1)
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
        DELETE FROM client_groups where name = ?1
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
}

async fn update_group(
    ctx: Extension<ApiContext>,
    Path(name): Path<String>,
    Json(req): Json<UpdateGroup>,
) -> ApiResult<Json<()>> {
    let mut conn = ctx.pool.acquire().await?;

    query!(
        r#"
        UPDATE client_groups
        SET name = ?1
        WHERE name = ?2
        "#,
        req.name,
        name
    )
    .execute(&mut conn)
    .await?;

    Ok(Json(()))
}
