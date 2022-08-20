use crate::web::util::{ApiContext, ApiContextAuth, ApiResult, JweService};
use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    response::IntoResponse,
};
use axum::{routing::post, Router};
use jsonwebtoken::EncodingKey;
use sqlx::SqlitePool;
use webauthn_rs::prelude::*;

/// Axum Example from here: https://github.com/kanidm/webauthn-rs/blob/master/tutorial/server/axum/src/auth.rs

pub fn router(pool: SqlitePool, jwe: JweService) -> Router {
    Router::new()
        .route("/api/auth/register_start/:username", post(start_register))
        .route("/api/auth/register_finish", post(finish_register))
        .route(
            "/api/auth/login_start/:username",
            post(start_authentication),
        )
        .route("/api/auth/login_finish", post(finish_authentication))
        .layer(Extension(ApiContextAuth { pool, jwe }))
}

pub async fn start_register(
    Extension(ctx): Extension<ApiContextAuth>,
    Path(username): Path<String>,
) -> ApiResult<impl IntoResponse> {
    tracing::info!("Start register");
    // We get the username from the URL, but you could get this via form submission or
    // some other process. In some parts of Webauthn, you could also use this as a "display name"
    // instead of a username. Generally you should consider that the user *can* and *will* change
    // their username at any time.

    // Since a user's username could change at anytime, we need to bind to a unique id.
    // We use uuid's for this purpose, and you should generate these randomly. If the
    // username does exist and is found, we can match back to our unique id. This is
    // important in authentication, where presented credentials may *only* provide
    // the unique id, and not the username!

    let user_unique_id = {
        let users_guard = app_state.users.lock().await;
        users_guard
            .name_to_id
            .get(&username)
            .copied()
            .unwrap_or_else(Uuid::new_v4)
    };

    // Remove any previous registrations that may have occured from the session.
    session.remove("reg_state");

    // If the user has any other credentials, we exclude these here so they can't be duplicate registered.
    // It also hints to the browser that only new credentials should be "blinked" for interaction.
    let exclude_credentials = {
        let users_guard = app_state.users.lock().await;
        users_guard
            .keys
            .get(&user_unique_id)
            .map(|keys| keys.iter().map(|sk| sk.cred_id().clone()).collect())
    };

    let res = match app_state.webauthn.start_passkey_registration(
        user_unique_id,
        &username,
        &username,
        exclude_credentials,
    ) {
        Ok((ccr, reg_state)) => {
            session
                .insert("reg_state", (username, user_unique_id, reg_state))
                .expect("Failed to insert");
            tracing::info!("Registration Successful!");
            Json(ccr)
        }
        Err(e) => {
            tracing::debug!("challenge_register -> {:?}", e);
            return Err(WebauthnError::Unknown);
        }
    };
    Ok(res)
}

pub async fn finish_register(
    Extension(ctx): Extension<ApiContext>,
    mut session: WritableSession,
    Json(reg): Json<RegisterPublicKeyCredential>,
) -> ApiResult<impl IntoResponse> {
    let (username, user_unique_id, reg_state): (String, Uuid, PasskeyRegistration) = session
        .get("reg_state")
        .ok_or(WebauthnError::CorruptSession)?; //Corrupt Session

    session.remove("reg_state");

    let res = match app_state
        .webauthn
        .finish_passkey_registration(&reg, &reg_state)
    {
        Ok(sk) => {
            let mut users_guard = app_state.users.lock().await;

            //TODO: This is where we would store the credential in a db, or persist them in some other way.
            users_guard
                .keys
                .entry(user_unique_id)
                .and_modify(|keys| keys.push(sk.clone()))
                .or_insert_with(|| vec![sk.clone()]);

            users_guard.name_to_id.insert(username, user_unique_id);

            StatusCode::OK
        }
        Err(e) => {
            tracing::debug!("challenge_register -> {:?}", e);
            StatusCode::BAD_REQUEST
        }
    };

    Ok(res)
}

pub async fn start_authentication(
    Extension(ctx): Extension<ApiContext>,
    mut session: WritableSession,
    Path(username): Path<String>,
) -> ApiResult<impl IntoResponse> {
    tracing::info!("Start Authentication");
    // We get the username from the URL, but you could get this via form submission or
    // some other process.

    // Remove any previous authentication that may have occured from the session.
    session.remove("auth_state");

    // Get the set of keys that the user possesses
    let users_guard = app_state.users.lock().await;

    // Look up their unique id from the username
    let user_unique_id = users_guard
        .name_to_id
        .get(&username)
        .copied()
        .ok_or(WebauthnError::UserNotFound)?;

    let allow_credentials = users_guard
        .keys
        .get(&user_unique_id)
        .ok_or(WebauthnError::UserHasNoCredentials)?;

    let res = match app_state
        .webauthn
        .start_passkey_authentication(allow_credentials)
    {
        Ok((rcr, auth_state)) => {
            // Drop the mutex to allow the mut borrows below to proceed
            drop(users_guard);

            session
                .insert("auth_state", (user_unique_id, auth_state))
                .expect("Failed to insert");
            Json(rcr)
        }
        Err(e) => {
            tracing::debug!("challenge_authenticate -> {:?}", e);
            return Err(WebauthnError::Unknown);
        }
    };
    Ok(res)
}

pub async fn finish_authentication(
    Extension(ctx): Extension<ApiContext>,
    mut session: WritableSession,
    Json(auth): Json<PublicKeyCredential>,
) -> ApiResult<impl IntoResponse> {
    let (user_unique_id, auth_state): (Uuid, PasskeyAuthentication) = session
        .get("auth_state")
        .ok_or(WebauthnError::CorruptSession)?;

    session.remove("auth_state");

    let res = match app_state
        .webauthn
        .finish_passkey_authentication(&auth, &auth_state)
    {
        Ok(auth_result) => {
            let mut users_guard = app_state.users.lock().await;

            // Update the credential counter, if possible.
            users_guard
                .keys
                .get_mut(&user_unique_id)
                .map(|keys| {
                    keys.iter_mut().for_each(|sk| {
                        // This will update the credential if it's the matching
                        // one. Otherwise it's ignored. That is why it is safe to
                        // iterate this over the full list.
                        sk.update_credential(&auth_result);
                    })
                })
                .ok_or(WebauthnError::UserHasNoCredentials)?;
            StatusCode::OK
        }
        Err(e) => {
            tracing::debug!("challenge_register -> {:?}", e);
            StatusCode::BAD_REQUEST
        }
    };
    tracing::info!("Authentication Successful!");
    Ok(res)
}
