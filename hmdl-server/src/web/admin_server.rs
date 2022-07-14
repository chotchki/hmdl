use crate::web::endpoints::{domain_groups, domains, health};
use axum::{handler::Handler, http::StatusCode, Router};
use sqlx::SqlitePool;
use std::{io, net::SocketAddr};

pub struct AdminServer;

impl AdminServer {
    pub async fn create(pool: SqlitePool) -> io::Result<()> {
        // build our application with a route
        let mut app = Router::new().fallback(fallback.into_service());

        app = app.merge(domains::router(pool.clone()));
        app = app.merge(domain_groups::router(pool.clone()));
        app = app.merge(health::router());

        //Only enable static content if we're in release mode
        #[cfg(not(debug_assertions))]
        {
            app = app.merge(crate::web::endpoints::frontend::router());
        }

        let addr = SocketAddr::from(([0, 0, 0, 0], 80));

        tracing::info!("Web Server listening on {}", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
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
