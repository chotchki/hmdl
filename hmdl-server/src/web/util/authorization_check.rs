use axum::extract::RequestParts;
use axum::http::{Request, StatusCode};
use axum::{extract::FromRequest, middleware::Next, response::Response};
use axum_sessions::extractors::ReadableSession;
use hmdl_db::dao::users::{Roles, User};

use crate::web::endpoints::authentication::USER;

pub async fn is_admin<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode>
where
    B: Send,
{
    let mut parts = RequestParts::new(req);
    let session = match ReadableSession::from_request(&mut parts).await {
        Ok(s) => s,
        Err(e) => {
            tracing::error!("No session found!");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    let user: User = match session.get(USER) {
        Some(s) => {
            tracing::debug!("Found user {:#?}", s);
            s
        }
        None => {
            tracing::error!("No user found!");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    if user.role != Roles::Admin {
        tracing::error!("User {} is not an admin", user.display_name);
        return Err(StatusCode::FORBIDDEN);
    }

    let back_into_req = parts.try_into_request().unwrap(); //This shouldn't fail

    Ok(next.run(back_into_req).await)
}
