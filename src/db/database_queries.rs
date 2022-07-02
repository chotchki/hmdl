use std::net::SocketAddr;

use async_recursion::async_recursion;
use sqlx::types::chrono::Utc;
use sqlx::{query, Connection, SqlitePool, Transaction};
use thiserror::Error;
use trust_dns_server::client::rr::LowerName;

pub struct DatabaseQueries;

impl DatabaseQueries {
    pub async fn log_domain(
        pool: &SqlitePool,
        domain: &LowerName,
        last_client: &SocketAddr,
    ) -> Result<(), DatabaseQueriesError> {
        let mut conn = pool.acquire().await?;
        let mut tran = conn.begin().await?;

        let timestamp = Utc::now();
        let client_str = last_client.to_string();

        if let Some(found_domain) = Self::domain_or_parent_exists(&mut tran, domain).await? {
            let domain_str = found_domain.to_string();
            query!(
                r#"
            INSERT INTO 
                known_domains (name, last_seen, last_client) 
            VALUES 
                (?1, ?2, ?3)
            ON CONFLICT(name) DO UPDATE SET
                last_seen=?2,
                last_client=?3
            "#,
                domain_str,
                timestamp,
                client_str
            )
            .execute(&mut tran)
            .await?;
        } else {
            let domain_str = domain.to_string();
            query!(
                r#"
                INSERT INTO 
                    known_domains (name, last_seen, last_client) 
                VALUES 
                    (?1, ?2, ?3)
                "#,
                domain_str,
                timestamp,
                client_str
            )
            .execute(&mut tran)
            .await?;
        }

        tran.commit().await?;

        Ok(())
    }

    #[async_recursion]
    pub async fn domain_or_parent_exists(
        transaction: &mut Transaction<'_, sqlx::Sqlite>,
        domain: &LowerName,
    ) -> Result<Option<LowerName>, DatabaseQueriesError> {
        let domain_str = domain.to_string();

        let found = query!(
            r#"
            SELECT EXISTS(SELECT 1 FROM known_domains WHERE name=?1) as exist
            "#,
            domain_str
        )
        .fetch_one(&mut *transaction)
        .await?;

        if found.exist == 1 {
            Ok(Some(domain.clone()))
        } else if domain.is_root() {
            Ok(None)
        } else {
            DatabaseQueries::domain_or_parent_exists(transaction, &domain.base_name()).await
        }
    }
}

pub async fn list_uncategorized_domains(
    pool: &SqlitePool,
) -> Result<Vec<String>, DatabaseQueriesError> {
    let mut conn = pool.acquire().await?;

    let domains = query!(
        r#"
        SELECT name
        FROM known_domains
        ORDER BY name
        "#
    )
    .fetch_all(&mut conn)
    .await?;

    let domain_vec = domains.into_iter().map(|x| x.name).collect();
    Ok(domain_vec)
}

#[derive(Debug, Error)]
pub enum DatabaseQueriesError {
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
}
