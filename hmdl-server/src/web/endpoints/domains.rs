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
        .route("/api/domains", get(list_uncat_domains))
        .route(
            "/api/domains/:name",
            delete(delete_domain).put(update_domain),
        )
        .route(
            "/api/domains/:name/group",
            delete(remove_domain_from_group).put(update_domain_group),
        )
        .layer(Extension(ApiContext { pool }))
}

async fn delete_domain(
    ctx: Extension<ApiContext>,
    Path(name): Path<String>,
) -> ApiResult<Json<()>> {
    let mut conn = ctx.pool.acquire().await?;

    query!(
        r#"
        DELETE FROM known_domains where name = ?1
        "#,
        name
    )
    .execute(&mut conn)
    .await?;

    Ok(Json(()))
}

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct Domain {
    pub name: String,
    pub last_seen: NaiveDateTime,
    pub last_client: String,
}

async fn list_uncat_domains(ctx: Extension<ApiContext>) -> ApiResult<Json<Vec<Domain>>> {
    let mut conn = ctx.pool.acquire().await?;

    let domains = query_as!(
        Domain,
        r#"
        SELECT name, last_seen, last_client
        FROM known_domains
        EXCEPT
        SELECT name, last_seen, last_client
        FROM known_domains
            INNER JOIN domain_group_member on domain_group_member.domain_name = known_domains.name
        WHERE 
            manually_set = true
        ORDER BY name
        "#
    )
    .fetch_all(&mut conn)
    .await?;

    Ok(Json(domains))
}

#[derive(Deserialize, Serialize)]
struct UpdateDomain {
    domain: Domain,
    group_name: String,
}

async fn update_domain(
    ctx: Extension<ApiContext>,
    Path(name): Path<String>,
    Json(req): Json<UpdateDomain>,
) -> ApiResult<Json<()>> {
    let mut conn = ctx.pool.acquire().await?;

    query!(
        r#"
        UPDATE known_domains
        SET name = ?1,
            last_seen = ?2,
            last_client = ?3
        WHERE name = ?4
        "#,
        req.domain.name,
        req.domain.last_seen,
        req.domain.last_client,
        name
    )
    .execute(&mut conn)
    .await?;

    Ok(Json(()))
}

async fn remove_domain_from_group(
    ctx: Extension<ApiContext>,
    Path(name): Path<String>,
) -> ApiResult<Json<()>> {
    let mut conn = ctx.pool.acquire().await?;

    query!(
        r#"
        DELETE FROM domain_group_member
        WHERE domain_name = ?1 
        "#,
        name,
    )
    .execute(&mut conn)
    .await?;

    Ok(Json(()))
}

async fn update_domain_group(
    ctx: Extension<ApiContext>,
    Path(name): Path<String>,
    Json(new_group_name): Json<String>,
) -> ApiResult<Json<()>> {
    let mut tran = ctx.pool.begin().await?;

    query!(
        r#"
        INSERT INTO domain_group_member(
            domain_name,
            group_name,
            manually_set
        )
        VALUES (
            ?1,
            ?2,
            true
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
