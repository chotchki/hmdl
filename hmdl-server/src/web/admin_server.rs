use crate::web::endpoints::{
    client_groups, clients, domain_groups, domains, groups_applied, health, setup,
};
use axum::{handler::Handler, http::StatusCode, Router};
use sqlx::SqlitePool;
use std::net::SocketAddr;
use tokio::sync::mpsc::unbounded_channel;

pub struct AdminServer {
    pool: SqlitePool,
}

impl AdminServer {
    pub fn create(pool: SqlitePool) -> AdminServer {
        let (sender, recv) = unbounded_channel::<()>();
        AdminServer { pool }
    }

    fn create_router(pool: SqlitePool) -> Router {
        let mut app = Router::new().fallback(fallback.into_service());

        app = app.merge(clients::router(pool.clone()));
        app = app.merge(client_groups::router(pool.clone()));
        app = app.merge(domains::router(pool.clone()));
        app = app.merge(domain_groups::router(pool.clone()));
        app = app.merge(groups_applied::router(pool.clone()));
        app = app.merge(health::router());
        app = app.merge(setup::router(pool));

        //Only enable static content if we're in release mode
        #[cfg(not(debug_assertions))]
        {
            app = app.merge(crate::web::endpoints::frontend::router());
        }

        app
    }

    pub async fn start(&self) -> Result<(), hyper::Error> {
        //Start up process
        //Do I have a domain and cloudflare api key?
        //What's my IP?
        //Prefer private over public
        //Check domain, does it match IP?
        //If not, update cloudflare

        let addr = SocketAddr::from(([0, 0, 0, 0], 80));

        let app_serv = Self::create_router(self.pool.clone()).into_make_service();

        tracing::info!("Web Server listening on {}", addr);
        axum::Server::bind(&addr).serve(app_serv).await?;

        Ok(())
    }
}

async fn fallback() -> (StatusCode, String) {
    (
        StatusCode::NOT_FOUND,
        "404 - Yeah you're not finding what you want.".to_string(),
    )
}
