use crate::web::util::{ApiContextAuth, ApiResult};
use axum::extract::{Extension, Json, Path};
use axum::{routing::post, Router};
use axum_sessions::{
    async_session::{serde_json, MemoryStore},
    extractors::WritableSession,
    SessionLayer,
};
use hmdl_db::dao::users::{self, Roles, User};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::sync::Arc;
use thiserror::Error;
use tower::ServiceBuilder;
use uuid::Uuid;
use webauthn_rs::{
    prelude::{
        Base64UrlSafeData, CreationChallengeResponse, PasskeyAuthentication, PasskeyRegistration,
        PublicKeyCredential, RegisterPublicKeyCredential, RequestChallengeResponse,
    },
    Webauthn,
};

/// Axum Example from here: https://github.com/kanidm/webauthn-rs/blob/master/tutorial/server/axum/src/auth.rs
const AUTH_SESSION: &str = "AUTH_SESSION";
const REG_SESSION: &str = "REG_SESSION";
pub const USER: &str = "USER";

pub fn router(
    pool: SqlitePool,
    session_layer: SessionLayer<MemoryStore>,
    webauthn: Arc<Webauthn>,
) -> Router {
    Router::new()
        .route("/api/auth/register_start/:username", post(start_register))
        .route("/api/auth/register_finish", post(finish_register))
        .route(
            "/api/auth/login_start/:username",
            post(start_authentication),
        )
        .route("/api/auth/login_finish", post(finish_authentication))
        .layer(ServiceBuilder::new().layer(session_layer))
        .layer(Extension(ApiContextAuth { pool, webauthn }))
}

#[derive(Debug, Deserialize, Serialize)]
struct RegistrationData {
    username: String,
    id: Uuid,
    state: PasskeyRegistration,
}

pub async fn start_register(
    Extension(ctx): Extension<ApiContextAuth>,
    Path(username): Path<String>,
    mut session: WritableSession,
) -> ApiResult<Json<CreationChallengeResponse>> {
    tracing::info!("Start register");

    session.regenerate();

    let user = match users::find_by_name(&ctx.pool, &username).await? {
        Some(s) => s,
        None => User {
            display_name: username.to_string(),
            id: Uuid::new_v4(),
            keys: vec![],
            role: Roles::Registered,
        },
    };

    let mut exclude = vec![];
    for key in user.keys {
        exclude.push(key.cred_id().clone());
    }

    let exclude: Option<Vec<Base64UrlSafeData>> = match exclude.len() {
        0 => None,
        _ => Some(exclude),
    };

    let (ccr, reg_state) = ctx.webauthn.start_passkey_registration(
        user.id,
        &user.display_name,
        &user.display_name,
        exclude,
    )?;

    session
        .insert(
            REG_SESSION,
            RegistrationData {
                username: user.display_name,
                id: user.id,
                state: reg_state,
            },
        )
        .map_err(AuthenticationError::Serde)?;

    Ok(Json(ccr))
}

pub async fn finish_register(
    ctx: Extension<ApiContextAuth>,
    mut session: WritableSession,
    Json(reg_pub_cred): Json<RegisterPublicKeyCredential>,
) -> ApiResult<Json<()>> {
    let reg_data: RegistrationData = session
        .get(REG_SESSION)
        .ok_or(AuthenticationError::RegistrationDataMissing)?;

    let res = ctx
        .webauthn
        .finish_passkey_registration(&reg_pub_cred, &reg_data.state)?;

    let new_user = User {
        display_name: reg_data.username,
        id: reg_data.id,
        keys: vec![res],
        role: Roles::Registered,
    };

    users::create(&ctx.pool, &new_user).await?;

    session
        .insert(USER, new_user)
        .map_err(AuthenticationError::Serde)?;

    Ok(Json(()))
}

#[derive(Debug, Deserialize, Serialize)]
struct AuthenticationData {
    username: String,
    id: Uuid,
    state: PasskeyAuthentication,
}

pub async fn start_authentication(
    Extension(ctx): Extension<ApiContextAuth>,
    Path(username): Path<String>,
    mut session: WritableSession,
) -> ApiResult<Json<RequestChallengeResponse>> {
    tracing::info!("Start Authentication");

    session.regenerate();

    let user = users::find_by_name(&ctx.pool, &username)
        .await?
        .ok_or(AuthenticationError::UserNotFound)?;

    let (rccr, auth_passkey) = ctx.webauthn.start_passkey_authentication(&user.keys)?;

    session
        .insert(
            AUTH_SESSION,
            AuthenticationData {
                username: user.display_name,
                id: user.id,
                state: auth_passkey,
            },
        )
        .map_err(AuthenticationError::Serde)?;

    Ok(Json(rccr))
}

pub async fn finish_authentication(
    Extension(ctx): Extension<ApiContextAuth>,
    Json(pub_cred): Json<PublicKeyCredential>,
    mut session: WritableSession,
) -> ApiResult<Json<()>> {
    let auth_data: AuthenticationData = session
        .get(AUTH_SESSION)
        .ok_or(AuthenticationError::AuthenticationDataMissing)?;

    let res = ctx
        .webauthn
        .finish_passkey_authentication(&pub_cred, &auth_data.state)?;

    let mut user = users::find_by_name(&ctx.pool, &auth_data.username)
        .await?
        .ok_or(AuthenticationError::UserNotFound)?;

    if res.needs_update() {
        user.keys.iter_mut().for_each(|sk| {
            sk.update_credential(&res);
        });

        users::update(&ctx.pool, &auth_data.username, &user).await?;
    }

    session
        .insert(USER, user)
        .map_err(AuthenticationError::Serde)?;

    Ok(Json(()))
}

#[derive(Debug, Error)]
pub enum AuthenticationError {
    #[error("Missing authentication data")]
    AuthenticationDataMissing,
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error("User not found")]
    UserNotFound,
    #[error("Missing registration data")]
    RegistrationDataMissing,
}
