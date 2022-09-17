use crate::web::util::{is_admin, ApiContext, ApiResult};
use axum::{
    extract::Path,
    routing::{delete, get},
    Extension, Json, Router,
};
use axum_sessions::{async_session::MemoryStore, SessionLayer};
use hmdl_db::dao::{
    roles::Roles,
    users::{self, User},
};
use sqlx::SqlitePool;
use strum::IntoEnumIterator;
use tower::ServiceBuilder;

pub fn router(pool: SqlitePool, session_layer: SessionLayer<MemoryStore>) -> Router {
    Router::new()
        .route("/api/roles", get(list_roles))
        .route("/api/users", get(list_users))
        .route("/api/users/:name", delete(delete_user).put(update_user))
        .layer(
            ServiceBuilder::new()
                .layer(Extension(ApiContext { pool }))
                .layer(session_layer)
                .layer(axum::middleware::from_fn(is_admin)),
        )
}

async fn list_roles() -> ApiResult<Json<Vec<Roles>>> {
    Ok(Json(Roles::iter().collect()))
}

async fn list_users(ctx: Extension<ApiContext>) -> ApiResult<Json<Vec<User>>> {
    let mut conn = ctx.pool.acquire().await?;

    let users = users::find_all(&mut conn).await?;

    Ok(Json(users))
}

async fn delete_user(ctx: Extension<ApiContext>, Path(name): Path<String>) -> ApiResult<Json<()>> {
    let mut conn = ctx.pool.acquire().await?;

    users::delete(&mut conn, &name).await?;

    Ok(Json(()))
}

async fn update_user(
    ctx: Extension<ApiContext>,
    Path(name): Path<String>,
    Json(user): Json<User>,
) -> ApiResult<Json<()>> {
    let mut conn = ctx.pool.acquire().await?;

    users::update(&mut conn, &name, &user).await?;

    Ok(Json(()))
}
