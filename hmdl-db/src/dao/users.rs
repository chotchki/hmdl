use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;
use webauthn_rs::prelude::Passkey;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Roles {
    Anonymous,
    Registered,
    Admin,
}

#[derive(Clone, Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct User {
    pub display_name: String,
    pub id: Uuid,
    pub keys: Vec<Passkey>,
    pub role: Roles,
}

pub async fn create(exec: impl sqlx::SqliteExecutor<'_>, user: &User) -> Result<(), UserError> {
    let id = user.id.to_string();
    let keys = serde_json::to_string(&user.keys)?;
    let role = serde_json::to_string(&user.role)?;

    sqlx::query!(
        r#"
        insert into users (
            display_name,
            id,
            keys,
            app_role
        ) VALUES (
            ?1,
            ?2,
            ?3,
            ?4
        )
        "#,
        user.display_name,
        id,
        keys,
        role
    )
    .execute(exec)
    .await?;

    Ok(())
}

pub async fn find_by_name(
    exec: impl sqlx::SqliteExecutor<'_>,
    display_name: &str,
) -> Result<Option<User>, UserError> {
    let res = sqlx::query!(
        r#"
        SELECT 
            display_name,
            id,
            keys,
            app_role
        FROM users
        WHERE
            display_name = ?1
    "#,
        display_name
    )
    .fetch_optional(exec)
    .await?;

    if let Some(s) = res {
        Ok(Some(User {
            display_name: s.display_name,
            id: Uuid::parse_str(&s.id).map_err(|_| UserError::Uuid(s.id.clone()))?,
            keys: serde_json::from_str(&s.keys)?,
            role: serde_json::from_str(&s.app_role)?,
        }))
    } else {
        Ok(None)
    }
}

pub async fn update(
    exec: impl sqlx::SqliteExecutor<'_>,
    display_name: &str,
    user: &User,
) -> Result<(), UserError> {
    let id = user.id.to_string();
    let keys = serde_json::to_string(&user.keys)?;
    let role = serde_json::to_string(&user.role)?;

    sqlx::query!(
        r#"
        update users
        set
            display_name = ?1,
            id = ?2,
            keys = ?3,
            app_role = ?4
        where
            display_name = ?5
        "#,
        user.display_name,
        id,
        keys,
        role,
        display_name
    )
    .execute(exec)
    .await?;

    Ok(())
}

#[derive(Debug, Error)]
pub enum UserError {
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error("Uuid parse error {0}")]
    Uuid(String),
}
