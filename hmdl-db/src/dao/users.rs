use super::roles::Roles;
use serde::{Deserialize, Serialize};
use sqlx::{
    query_as,
    sqlite::{SqliteQueryResult, SqliteRow},
    Error::ColumnDecode,
    FromRow, Row,
};
use std::str::FromStr;
use strum::ParseError;
use thiserror::Error;
use uuid::Uuid;
use webauthn_rs::prelude::Passkey;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub display_name: String,
    pub id: Uuid,
    pub keys: Vec<Passkey>,
    pub role: Roles,
}

impl FromRow<'_, SqliteRow> for User {
    fn from_row(row: &SqliteRow) -> sqlx::Result<Self> {
        Ok(Self {
            display_name: row.try_get("display_name")?,
            id: Uuid::parse_str(row.try_get("id")?).map_err(|e| ColumnDecode {
                index: "id".to_string(),
                source: Box::new(e),
            })?,
            keys: serde_json::from_str(row.try_get("keys")?).map_err(|e| ColumnDecode {
                index: "keys".to_string(),
                source: Box::new(e),
            })?,
            role: Roles::from_str(row.try_get("app_role")?).map_err(|e| ColumnDecode {
                index: "app_role".to_string(),
                source: Box::new(e),
            })?,
        })
    }
}

pub async fn create(exec: impl sqlx::SqliteExecutor<'_>, user: &mut User) -> Result<(), UserError> {
    let id = user.id.to_string();
    let keys = serde_json::to_string(&user.keys)?;

    //Handling the fist user
    let rec = sqlx::query!(
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
            CASE WHEN (SELECT COUNT(*) from users) > 0
            THEN 'Admin'
            ELSE 'Registered'
            END
        )
        RETURNING app_role
        "#,
        user.display_name,
        id,
        keys
    )
    .fetch_one(exec)
    .await?;

    user.role = Roles::from_str(&rec.app_role)?;

    Ok(())
}

pub async fn delete(
    exec: impl sqlx::SqliteExecutor<'_>,
    display_name: &str,
) -> Result<SqliteQueryResult, UserError> {
    sqlx::query!(
        r#"
        delete from users
        where
            display_name = ?1
        "#,
        display_name
    )
    .execute(exec)
    .await
    .map_err(UserError::Sqlx)
}

pub async fn find_by_name(
    exec: impl sqlx::SqliteExecutor<'_>,
    display_name: &str,
) -> Result<Option<User>, UserError> {
    query_as(
        r#"
        SELECT 
            display_name,
            id,
            keys,
            app_role as role
        FROM users
        WHERE
            display_name = ?1
    "#,
    )
    .bind(display_name)
    .fetch_optional(exec)
    .await
    .map_err(UserError::Sqlx)
}

pub async fn find_all(exec: impl sqlx::SqliteExecutor<'_>) -> Result<Vec<User>, UserError> {
    query_as(
        r#"
        SELECT 
            display_name,
            id,
            keys,
            app_role as role
        FROM users
    "#,
    )
    .fetch_all(exec)
    .await
    .map_err(UserError::Sqlx)
}

pub async fn update(
    exec: impl sqlx::SqliteExecutor<'_>,
    display_name: &str,
    user: &User,
) -> Result<SqliteQueryResult, UserError> {
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
    .await
    .map_err(UserError::Sqlx)
}

#[derive(Debug, Error)]
pub enum UserError {
    #[error(transparent)]
    StrumError(#[from] ParseError),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}
