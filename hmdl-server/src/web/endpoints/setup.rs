use crate::web::util::{ApiContext, ApiResult};
use axum::{routing::get, Extension, Json, Router};
use serde::Serialize;
use sqlx::{query, SqlitePool};
use tower::builder::ServiceBuilder;

pub fn router(pool: SqlitePool) -> Router {
    Router::new()
        .route("/api/is-setup", get(is_setup))
        .layer(ServiceBuilder::new().layer(Extension(ApiContext { pool })))
}

#[derive(Serialize)]
struct SetupStatusResp {
    status: String,
    domain: Option<String>,
}

// Api path confirming that the application is not setup
async fn is_setup(ctx: Extension<ApiContext>) -> ApiResult<Json<SetupStatusResp>> {
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
