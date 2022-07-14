use crate::web::endpoints::{domain_groups, domains, frontend, health};
use axum::{handler::Handler, http::StatusCode, Router};
use sqlx::SqlitePool;
use std::{io, net::SocketAddr};

pub struct AdminServer;

impl AdminServer {
    pub async fn create(pool: SqlitePool) -> io::Result<()> {
        // build our application with a route
        let app = Router::new().fallback(fallback.into_service());

        let merge_app = app
            .merge(domains::router(pool.clone()))
            .merge(frontend::router())
            .merge(domain_groups::router(pool.clone()))
            .merge(health::router());

        let addr = SocketAddr::from(([0, 0, 0, 0], 80));

        tracing::info!("Web Server listening on {}", addr);
        axum::Server::bind(&addr)
            .serve(merge_app.into_make_service())
            .await
            .unwrap();

        Ok(())
    }
}

async fn fallback() -> (StatusCode, String) {
    (
        StatusCode::NOT_FOUND,
        "404 - Yeah you're not finding what you want.".to_string(),
    )
}
