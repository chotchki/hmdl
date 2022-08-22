use std::sync::Arc;

use crate::web::util::{ApiContext, ApiContextAuth, ApiResult, JweService};
use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    response::IntoResponse, TypedHeader,
};
use axum::{routing::post, Router};
use hmdl_db::dao::users::{self, Roles, User};
use serde::{Serialize, Deserialize};
use sqlx::SqlitePool;
use thiserror::Error;
use webauthn_rs::prelude::*;

/// Axum Example from here: https://github.com/kanidm/webauthn-rs/blob/master/tutorial/server/axum/src/auth.rs

pub fn router(pool: SqlitePool, jwe: JweService, webauthn: Arc<Webauthn>) -> Router {
    Router::new()
        .route("/api/auth/register_start/:username", post(start_register))
        .route("/api/auth/register_finish", post(finish_register))
        .route(
            "/api/auth/login_start/:username",
            post(start_authentication),
        )
        .route("/api/auth/login_finish", post(finish_authentication))
        .layer(Extension(ApiContextAuth { pool, jwe, webauthn }))
}

#[derive(Debug, Serialize)]
struct RegistrationResponse {
    ccr: CreationChallengeResponse,
    reg_token: String
}

pub async fn start_register(
    Extension(ctx): Extension<ApiContextAuth>,
    Path(username): Path<String>,
) -> ApiResult<Json<RegistrationResponse>> {
    tracing::info!("Start register");

    let user = match users::findByName(&ctx.pool,&username).await? {
        Some(s) => s,
        None => User{
            display_name: username.to_string(),
            id: Uuid::new_v4(),
            keys: vec![],
            role: Roles::Registered
        }
    };

    let exclude = match user.keys.len() {
        0 => None,
        _ => Some(user.keys.iter().map(|sk| sk.cred_id().clone()).collect())
    };

    let (ccr, reg_state) = ctx.webauthn.start_passkey_registration(user.id, &user.display_name, &user.display_name, exclude)?;

    let reg_token = ctx.jwe.encrypt_registration_token(user.display_name, user.id, reg_state)?;

    Ok(Json(RegistrationResponse{
        ccr,
        reg_token
    }))
}

#[derive(Debug, Deserialize)]
struct FinishRegistrationRequest {
    reg_pub_cred: RegisterPublicKeyCredential,
    token: String
}

pub async fn finish_register(
    ctx: Extension<ApiContextAuth>,
    Json(reg): Json<FinishRegistrationRequest>,
) -> ApiResult<Json<()>> {
    let claims = ctx.jwe.decrypt_registration_token(reg.token)?;

    let res = ctx.webauthn.finish_passkey_registration(&reg.reg_pub_cred, &claims.reg_passkey)?;

    let new_user = User {
        display_name: claims.username,
        id: claims.unique_id,
        keys: vec![res],
        role: Roles::Registered
    };

    users::create(&ctx.pool, &new_user).await?;

    Ok(Json(()))
}

#[derive(Debug, Serialize)]
struct AuthenticationResponse {
    rccr: RequestChallengeResponse,
    auth_token: String
}

pub async fn start_authentication(
    Extension(ctx): Extension<ApiContextAuth>,
    Path(username): Path<String>,
) -> ApiResult<Json<AuthenticationResponse>> {
    tracing::info!("Start Authentication");

    let user = users::findByName(&ctx.pool,&username).await?.ok_or(AuthenticationError::UserNotFound)?;

    let (rccr, auth_passkey) = ctx.webauthn.start_passkey_authentication(&user.keys)?;

    let auth_token = ctx.jwe.encrypt_authentication_token(user.display_name, user.id, auth_passkey)?;

    Ok(Json(AuthenticationResponse{
        rccr,
        auth_token
    }))
}

#[derive(Debug, Deserialize)]
struct FinishAuthenticationRequest {
    pub_cred: PublicKeyCredential,
    token: String
}

pub async fn finish_authentication(
    Extension(ctx): Extension<ApiContextAuth>,
    Json(auth): Json<FinishAuthenticationRequest>,
) -> ApiResult<Json<String>> {
    let claims = ctx.jwe.decrypt_authentication_token(auth.token)?;

    let res = ctx.webauthn.finish_passkey_authentication(&auth.pub_cred, &claims.auth_passkey)?;

    if res.needs_update() {
        let mut user = users::findByName(&ctx.pool,&claims.username).await?.ok_or(AuthenticationError::UserNotFound)?;
        user.keys.iter_mut().for_each(|sk| { sk.update_credential(&res);});

        users::update(&ctx.pool, &claims.username, &user).await?;
    }

    
}

#[derive(Debug, Error)]
pub enum AuthenticationError {
    #[error("User not found")]
    UserNotFound,
}