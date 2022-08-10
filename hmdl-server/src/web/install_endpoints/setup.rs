use axum::{
    routing::{get, post},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{query, SqlitePool};
use tokio::sync::broadcast::Sender;
use tower::builder::ServiceBuilder;

use crate::web::util::{ApiContextSetup, ApiResult};

pub fn router(pool: SqlitePool, install_refresh_sender: Sender<()>) -> Router {
    let router = Router::new().route("/api/is-setup", get(is_setup));

    let setup_router =
        Router::new()
            .route("/api/setup", post(add_setup))
            .layer(ServiceBuilder::new().layer(Extension(ApiContextSetup {
                pool,
                install_refresh_sender,
            })));

    router.merge(setup_router)
}

#[derive(Serialize)]
struct SetupStatusResp {
    status: String,
    domain: Option<String>,
}

// Api path confirming that the application is not setup
async fn is_setup() -> ApiResult<Json<String>> {
    Ok(Json("Not Setup".to_string()))
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
            lock_column
        ) VALUES (
            ?1,
            ?2,
            ?3,
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

    ctx.install_refresh_sender.send(())?;

    tracing::info!("Setup Complete, switching into run mode");

    Ok(Json(()))
}
