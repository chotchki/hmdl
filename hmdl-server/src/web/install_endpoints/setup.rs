use crate::web::util::{ApiContextSetup, ApiResult};
use axum::{
    routing::{get, post},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{query, SqlitePool};
use tokio::sync::broadcast::Sender;
use tower::builder::ServiceBuilder;

pub fn router(pool: SqlitePool, install_refresh_sender: Sender<()>) -> Router {
    Router::new()
        .route("/api/is-setup", get(is_setup))
        .route("/api/setup", post(add_setup))
        .layer(ServiceBuilder::new().layer(Extension(ApiContextSetup {
            pool,
            install_refresh_sender,
        })))
}

#[derive(Serialize)]
struct SetupStatusResp {
    status: String,
    domain: Option<String>,
}

// Api path confirming that the application is not setup
async fn is_setup(ctx: Extension<ApiContextSetup>) -> ApiResult<Json<SetupStatusResp>> {
    let mut conn = ctx.pool.acquire().await?;

    let setting_record = query!(
        r#"
        SELECT application_domain, https_started_once
        FROM hmdl_settings
        WHERE lock_column == true
        "#
    )
    .fetch_optional(&mut conn)
    .await?;

    let status = match setting_record {
        None => SetupStatusResp {
            status: "Not Setup".to_string(),
            domain: None,
        },
        Some(s) => match s.https_started_once {
            false => SetupStatusResp {
                status: "In Progress".to_string(),
                domain: Some(s.application_domain),
            },
            true => SetupStatusResp {
                status: "Setup".to_string(),
                domain: Some(s.application_domain),
            },
        },
    };

    Ok(Json(status))
}

#[derive(Deserialize)]
pub struct HmdlSetup {
    pub application_domain: String,
    pub cloudflare_api_token: String,
    pub acme_email: String,
}

async fn add_setup(
    ctx: Extension<ApiContextSetup>,
    Json(setup): Json<HmdlSetup>,
) -> ApiResult<Json<()>> {
    let mut conn = ctx.pool.acquire().await?;

    query!(
        r#"
        INSERT OR REPLACE INTO hmdl_settings (
            application_domain, 
            cloudflare_api_token,
            acme_email,
            https_started_once,
            lock_column
        ) VALUES (
            ?1,
            ?2,
            ?3,
            false,
            true
        ) ON CONFLICT (lock_column) 
        DO UPDATE 
        SET 
            application_domain = ?1,
            cloudflare_api_token = ?2,
            acme_email = ?3
        "#,
        setup.application_domain,
        setup.cloudflare_api_token,
        setup.acme_email
    )
    .execute(&mut conn)
    .await?;

    tracing::info!("Setup Complete, switching into in progress mode");
    ctx.install_refresh_sender.send(())?;

    Ok(Json(()))
}
