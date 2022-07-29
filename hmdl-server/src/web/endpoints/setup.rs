use axum::{
    middleware::{self, Next},
    response::Response,
    routing::{get, post},
    Extension, Json, Router,
};
use hyper::{Request, StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::{query, SqliteConnection, SqlitePool};
use tower::builder::ServiceBuilder;

use crate::web::util::{ApiContext, ApiResult};

pub fn router(pool: SqlitePool) -> Router {
    let router = Router::new()
        .route("/api/is-setup", get(is_setup))
        .layer(Extension(ApiContext { pool: pool.clone() }));

    let setup_router = Router::new().route("/api/setup", post(add_setup)).layer(
        ServiceBuilder::new()
            .layer(Extension(ApiContext { pool }))
            .layer(middleware::from_fn(first_time_setup_check)),
    );

    router.merge(setup_router)
}

enum SetupStatus {
    Setup,
    NotSetup,
    InProgress,
}

#[derive(Serialize)]
struct SetupStatusResp {
    status: String,
    domain: Option<String>,
}

async fn is_setup(ctx: Extension<ApiContext>) -> ApiResult<Json<SetupStatusResp>> {
    let mut conn = ctx.pool.acquire().await?;

    match setup_status_db_check(&mut conn).await? {
        (SetupStatus::Setup, s) => Ok(Json(SetupStatusResp {
            status: "Setup".to_string(),
            domain: s,
        })),
        (SetupStatus::NotSetup, s) => Ok(Json(SetupStatusResp {
            status: "Not Setup".to_string(),
            domain: s,
        })),
        (SetupStatus::InProgress, s) => Ok(Json(SetupStatusResp {
            status: "In Progress".to_string(),
            domain: s,
        })),
    }
}

/// This function locks down the setup paths AFTER the setup process has completed,
/// will harden this as the process gets more flushed out
async fn first_time_setup_check<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let exts = req.extensions();
    let ctx_opt = exts.get::<ApiContext>();
    let pool = &ctx_opt.ok_or(StatusCode::INTERNAL_SERVER_ERROR)?.pool;

    let mut conn = pool
        .acquire()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match setup_status_db_check(&mut conn).await {
        Ok((SetupStatus::NotSetup, _)) => Ok(next.run(req).await),
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

async fn setup_status_db_check(
    conn: &mut SqliteConnection,
) -> ApiResult<(SetupStatus, Option<String>)> {
    let setting_record = query!(
        r#"
        SELECT application_domain
        FROM hmdl_settings
        WHERE lock_column == true
        "#
    )
    .fetch_optional(conn)
    .await?;

    if let Some(rec) = setting_record {
        Ok((SetupStatus::Setup, Some(rec.application_domain)))
    } else {
        Ok((SetupStatus::NotSetup, None))
    }
}

#[derive(Deserialize)]
pub struct HmdlSetup {
    pub application_domain: String,
    pub cloudflare_api_token: String,
}

async fn add_setup(
    ctx: Extension<ApiContext>,
    Json(setup): Json<HmdlSetup>,
) -> ApiResult<Json<()>> {
    let mut conn = ctx.pool.acquire().await?;

    query!(
        r#"
        INSERT OR REPLACE INTO hmdl_settings (
            application_domain, 
            cloudflare_api_token, 
            lock_column
        ) VALUES (
            ?1,
            ?2,
            true
        ) ON CONFLICT (lock_column) 
        DO UPDATE 
        SET 
            application_domain = ?1,
            cloudflare_api_token = ?2
        "#,
        setup.application_domain,
        setup.cloudflare_api_token,
    )
    .execute(&mut conn)
    .await?;

    Ok(Json(()))
}
