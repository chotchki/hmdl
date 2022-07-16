use axum::{
    extract::Path,
    routing::{delete, get},
    Extension, Json, Router,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, SqlitePool};

use crate::web::util::{ApiContext, ApiResult};

pub fn router(pool: SqlitePool) -> Router {
    Router::new()
        .route("/api/clients", get(list_uncat_clients))
        .route(
            "/api/clients/:name",
            delete(delete_client).put(update_client),
        )
        .route(
            "/api/clients/:name/group",
            delete(remove_client_from_group).put(update_client_group),
        )
        .layer(Extension(ApiContext { pool }))
}

async fn delete_client(
    ctx: Extension<ApiContext>,
    Path(name): Path<String>,
) -> ApiResult<Json<()>> {
    let mut conn = ctx.pool.acquire().await?;

    query!(
        r#"
        DELETE FROM clients where name = ?1
        "#,
        name
    )
    .execute(&mut conn)
    .await?;

    Ok(Json(()))
}

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct Client {
    pub name: String,
    pub ipv4: IpAddr,
    pub mac: String,
}

async fn list_uncat_clients(ctx: Extension<ApiContext>) -> ApiResult<Json<Vec<Domain>>> {
    let mut conn = ctx.pool.acquire().await?;

    let domains = query_as!(
        Domain,
        r#"
        SELECT name, ip, mac
        FROM clients
        EXCEPT
        SELECT name, ip, mac
        FROM clients
            INNER JOIN client_group_member on client_group_member.client_name = clients.name
        ORDER BY name
        "#
    )
    .fetch_all(&mut conn)
    .await?;

    Ok(Json(domains))
}

#[derive(Deserialize, Serialize)]
struct UpdateClient {
    client: Client,
    group_name: String,
}

async fn update_domain(
    ctx: Extension<ApiContext>,
    Path(name): Path<String>,
    Json(req): Json<Client>,
) -> ApiResult<Json<()>> {
    let mut conn = ctx.pool.acquire().await?;

    query!(
        r#"
        UPDATE clients
        SET name = ?1,
            ip = ?2,
            mac = ?3
        WHERE name = ?4
        "#,
        req.name,
        req.ip,
        req.mac,
        name
    )
    .execute(&mut conn)
    .await?;

    Ok(Json(()))
}

async fn remove_client_from_group(
    ctx: Extension<ApiContext>,
    Path(name): Path<String>,
) -> ApiResult<Json<()>> {
    let mut conn = ctx.pool.acquire().await?;

    query!(
        r#"
        DELETE FROM client_group_member
        WHERE client_name = ?1 
        "#,
        name,
    )
    .execute(&mut conn)
    .await?;

    Ok(Json(()))
}

async fn update_client_group(
    ctx: Extension<ApiContext>,
    Path(name): Path<String>,
    Json(new_group_name): Json<String>,
) -> ApiResult<Json<()>> {
    let mut tran = ctx.pool.begin().await?;

    query!(
        r#"
        DELETE FROM client_group_member
        WHERE client_name = ?1 
        "#,
        name,
    )
    .execute(&mut tran)
    .await?;

    query!(
        r#"
        INSERT INTO client_group_member(
            client_name,
            group_name
        )
        VALUES (
            ?1,
            ?2
        )
        "#,
        name,
        new_group_name,
    )
    .execute(&mut tran)
    .await?;

    tran.commit().await?;

    Ok(Json(()))
}
