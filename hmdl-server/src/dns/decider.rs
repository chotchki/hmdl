//This is the core of the filtering system, should we proceed or not?

//Note: Any failure in here will result in an allow so we don't block access

use std::net::IpAddr;

use async_recursion::async_recursion;
use chrono::Utc;
use sqlx::{query, Acquire, SqlitePool, Transaction};
use thiserror::Error;
use trust_dns_server::client::rr::LowerName;

pub enum Decision {
    Allow,
    Block,
}

//We absorb all errors here since this is the decision point of what to do
pub async fn should_filter(pool: SqlitePool, client: &IpAddr, domain: &LowerName) -> Decision {
    match should_filter_int(pool, client, domain).await {
        Ok(x) => return x,
        Err(e) => {
            tracing::error!("Failure of the filtering code {}", e);
            return Decision::Allow;
        }
    }
}

async fn should_filter_int(
    pool: SqlitePool,
    client: &IpAddr,
    domain: &LowerName,
) -> Result<Decision, DecisionError> {
    log_domain(&pool, domain, client).await?;

    Ok(Decision::Allow)
}

pub async fn log_domain(
    pool: &SqlitePool,
    domain: &LowerName,
    last_client: &IpAddr,
) -> Result<(), DecisionError> {
    let mut conn = pool.acquire().await?;
    let mut tran = conn.begin().await?;

    let timestamp = Utc::now();
    let client_str = last_client.to_string();

    if let Some(found_domain) = domain_or_parent_exists(&mut tran, domain).await? {
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
) -> Result<Option<LowerName>, DecisionError> {
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
        domain_or_parent_exists(transaction, &domain.base_name()).await
    }
}

#[derive(Debug, Error)]
pub enum DecisionError {
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
}
