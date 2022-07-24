//This is the core of the filtering system, should we proceed or not?

//Note: Any system failure in here will result in an allow so we don't block access

use chrono::Utc;
use sqlx::{query, query_as, SqlitePool};
use std::net::IpAddr;
use thiserror::Error;
use trust_dns_server::client::rr::LowerName;

use crate::web::endpoints::domains::Domain;

use super::arp_lookup::{self, ArpError};

pub enum Decision {
    Allow,
    Block,
}

//We absorb all errors here since this is the decision point of what to do
pub async fn should_filter(pool: SqlitePool, client: &IpAddr, domain: &LowerName) -> Decision {
    match should_filter_int(pool, client, domain).await {
        Ok(x) => x,
        Err(e) => {
            tracing::error!("Failure of the filtering code {}", e);
            Decision::Allow
        }
    }
}

async fn should_filter_int(
    pool: SqlitePool,
    client: &IpAddr,
    domain: &LowerName,
) -> Result<Decision, DecisionError> {
    log_client(&pool, client).await?;

    let client_str = client.to_string();
    let domain = log_domain(&pool, domain, client).await?;

    //Logging complete, let's make decisions
    let mut conn = pool.acquire().await?;
    let blocked = query!(
        r#"
        SELECT
            coalesce(
                (
                    SELECT 1
                    FROM (
                        SELECT name
                        FROM known_domains
                        WHERE name = ?1
                        EXCEPT
                        SELECT domain_name
                        FROM domain_group_member
                        WHERE domain_name = ?1
                    )
                ),
                (
                    SELECT 1
                    FROM known_domains
                    INNER JOIN domain_group_member ON known_domains.name = domain_group_member.domain_name
                    INNER JOIN groups_applied ON groups_applied.domain_group_name = domain_group_member.group_name
                    INNER JOIN client_group_member ON groups_applied.client_group_name = client_group_member.group_name
                    INNER JOIN clients ON clients.name = client_group_member.client_name
                    WHERE known_domains.name=?1
                    and clients.ip = ?2
                )
            ) as block
        "#,
        domain.name,
        client_str
    ).fetch_one(&mut conn).await?;

    if blocked.block == Some(1) {
        Ok(Decision::Block)
    } else {
        Ok(Decision::Allow)
    }
}

async fn log_client(pool: &SqlitePool, client: &IpAddr) -> Result<(), DecisionError> {
    let mut conn = pool.acquire().await?;

    let (mut hostname, mac) = arp_lookup::lookup_mac(client).await?;

    if hostname == "?" {
        hostname = mac.clone();
    }

    let client_str = client.to_string();

    query!(
        r#"
        INSERT INTO clients (
            name, ip, mac
        ) VALUES (
            ?1, ?2, ?3
        ) ON CONFLICT(name) DO UPDATE SET
            ip=?2,
            mac=?3
        "#,
        hostname,
        client_str,
        mac
    )
    .execute(&mut conn)
    .await?;

    Ok(())
}

async fn log_domain(
    pool: &SqlitePool,
    domain: &LowerName,
    last_client: &IpAddr,
) -> Result<Domain, DecisionError> {
    let mut conn = pool.acquire().await?;

    let domain_str = domain.to_string();

    let timestamp = Utc::now();
    let client_str = last_client.to_string();

    let domain = query_as!(
        Domain,
        r#"
        WITH RECURSIVE
            known(depth, domain_exists, domain_nm) AS (
                VALUES (0, 1, ?1)
                UNION ALL
                SELECT
                    k.depth+1,
                    EXISTS(
                        SELECT 1
                        FROM known_domains kn
                        WHERE kn.name=substring(k.domain_nm, (length(k.domain_nm) - instr(k.domain_nm,'.')) * -1)
                    ),
                    substring(k.domain_nm, (length(k.domain_nm) - instr(k.domain_nm,'.')) * -1)
                FROM
                known k
                WHERE length(substring(k.domain_nm, (length(k.domain_nm) - instr(k.domain_nm,'.')) * -1)) > 1
                and substring(k.domain_nm, (length(k.domain_nm) - instr(k.domain_nm,'.')) * -1) != k.domain_nm
        )
        INSERT OR REPLACE INTO known_domains (name, last_seen, last_client)
        SELECT 
            domain_nm, ?2, ?3
        FROM 
            known
        WHERE 
            domain_exists = 1
        ORDER BY 
            depth desc
        LIMIT 1
        ON CONFLICT(name) 
        DO UPDATE SET
            last_seen=?2,
            last_client=?3
        RETURNING name, last_seen, last_client
        "#,
        domain_str,
        timestamp,
        client_str
    ).fetch_one(&mut conn).await?;

    Ok(domain)
}

#[derive(Debug, Error)]
pub enum DecisionError {
    #[error(transparent)]
    ArpError(#[from] ArpError),
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
}

/*
// Find the parent record
WITH RECURSIVE
    known(depth, domain_exists, domain_nm) AS (
        VALUES (0, 1, 'tt.client.example.com.')
        UNION ALL
        SELECT
            k.depth+1,
            EXISTS(
                SELECT 1
                FROM known_domains kn
                WHERE kn.name=substring(k.domain_nm, (length(k.domain_nm) - instr(k.domain_nm,'.')) * -1)
            ),
            substring(k.domain_nm, (length(k.domain_nm) - instr(k.domain_nm,'.')) * -1)
        FROM
        known k
        WHERE length(substring(k.domain_nm, (length(k.domain_nm) - instr(k.domain_nm,'.')) * -1)) > 1
        and substring(k.domain_nm, (length(k.domain_nm) - instr(k.domain_nm,'.')) * -1) != k.domain_nm
)
SELECT domain_nm
FROM known
WHERE domain_exists = 1
ORDER BY depth desc
LIMIT 1;

WITH RECURSIVE
    known(depth, domain_exists, domain_nm) AS (
        VALUES (0, 1, 'tt.client.example.com.')
        UNION ALL
        SELECT
            k.depth+1,
            EXISTS(
                SELECT 1
                FROM known_domains kn
                WHERE kn.name=substring(k.domain_nm, (length(k.domain_nm) - instr(k.domain_nm,'.')) * -1)
            ),
            substring(k.domain_nm, (length(k.domain_nm) - instr(k.domain_nm,'.')) * -1)
        FROM
        known k
        WHERE length(substring(k.domain_nm, (length(k.domain_nm) - instr(k.domain_nm,'.')) * -1)) > 1
        and substring(k.domain_nm, (length(k.domain_nm) - instr(k.domain_nm,'.')) * -1) != k.domain_nm
)
INSERT OR REPLACE INTO known_domains (name, last_seen, last_client)
SELECT
    domain_nm, date(), 'Test-CTE2'
FROM
    known
WHERE
    domain_exists = 1
ORDER BY
    depth desc
LIMIT 1
ON CONFLICT(name)
DO UPDATE SET
    last_seen=date(),
    last_client='Test-CTE2'
;

WITH RECURSIVE
    known(depth, domain_exists, domain_nm) AS (
        VALUES (0, 1, 'tt.example.co.uk.')
        UNION ALL
        SELECT
            k.depth+1,
            EXISTS(
                SELECT 1
                FROM known_domains kn
                WHERE kn.name=substring(k.domain_nm, (length(k.domain_nm) - instr(k.domain_nm,'.')) * -1)
            ),
            substring(k.domain_nm, (length(k.domain_nm) - instr(k.domain_nm,'.')) * -1)
        FROM
        known k
        WHERE length(substring(k.domain_nm, (length(k.domain_nm) - instr(k.domain_nm,'.')) * -1)) > 1
        and substring(k.domain_nm, (length(k.domain_nm) - instr(k.domain_nm,'.')) * -1) != k.domain_nm
)
INSERT OR REPLACE INTO known_domains (name, last_seen, last_client)
SELECT
    domain_nm, date(), 'Test-CTE5'
FROM
    known
WHERE
    domain_exists = 1
ORDER BY
    depth desc
LIMIT 1
ON CONFLICT(name)
DO UPDATE SET
    last_seen=date(),
    last_client='Test-CTE5'
RETURNING name, last_seen, last_client
;

//Uncat domains are always blocked
// domains that are tied together are blocked

insert into domain_groups values('domain_test', 'NEW');
insert into domain_group_member values('example.com.', 'domain_test', 'N', null);
insert into clients values ('localhost', '127.0.0.1', '00:00:00:00:00:00');
insert into client_groups values ('kids');
insert into client_group_member values('localhost', 'kids');
insert into groups_applied values ('kids', 'domain_test');


            SELECT
            coalesce(
                    (SELECT 1
                    FROM (
                        SELECT name
                        FROM known_domains
                        WHERE name = 'example.com.'
                        EXCEPT
                        SELECT domain_name
                        FROM domain_group_member
                        WHERE domain_name = 'example.com.'
                    )),
                    (SELECT 1
                    FROM known_domains
                    INNER JOIN domain_group_member ON known_domains.name = domain_group_member.domain_name
                    INNER JOIN groups_applied ON groups_applied.domain_group_name = domain_group_member.group_name
                    INNER JOIN client_group_member ON groups_applied.client_group_name = client_group_member.group_name
                    INNER JOIN clients ON clients.name = client_group_member.client_name
                    WHERE known_domains.name='example.com.'
                    and clients.ip = '127.0.0.1')
            ) as block;

SELECT
                EXISTS(
                    SELECT 1
                    FROM (
                        SELECT name
                        FROM known_domains
                        WHERE name = 'example.com.'
                        EXCEPT
                        SELECT domain_name
                        FROM domain_group_member
                        WHERE domain_name = 'example.com.'
                    )
                );

//Bad
        SELECT
            coalesce(
                EXISTS(
                    SELECT 1
                    FROM (
                        SELECT name
                        FROM known_domains
                        WHERE name = ?1
                        EXCEPT
                        SELECT domain_name
                        FROM domain_group_member
                        WHERE domain_name = ?1
                    )
                ),
                EXISTS(
                    SELECT 1
                    FROM known_domains
                    INNER JOIN domain_group_member ON known_domains.name = domain_group_member.domain_name
                    INNER JOIN groups_applied ON groups_applied.domain_group_name = domain_group_member.group_name
                    INNER JOIN client_group_member ON groups_applied.client_group_name = client_group_member.group_name
                    INNER JOIN clients ON clients.name = client_group_member.client_name
                    WHERE known_domains.name=?1
                    and clients.ip = ?2
                )
            ) as block
*/
