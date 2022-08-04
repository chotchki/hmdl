pub mod setup;

const PORT: u16 = 53;

pub struct InstallEndpoints {
    pool: SqlitePool,
}

impl InstallEndpoints {
    pub fn create(pool: SqlitePool) -> Self {
        InstallEndpoints { pool }
    }

    pub async fn start(
        &self,
        mut ip_changed: Receiver<HashSet<IpAddr>>,
        settings_changed: Sender<HashSet<IpAddr>>
    ) -> Result<(), DnsServerError> {
        let mut ips = ip_changed.recv().await?;

        loop {
            let futures = FuturesUnordered::new();

            for ip in ips {
                tracing::info!("HTTP Install Server listening on {}", ip);
                let addr = SocketAddr::from(ip, PORT);
                let app_service = Self::create_router(self.pool.clone()).into_make_service();

                let builder = axum_server::bind(addr);
                future.push(builder.server(app_serv));
            }

            tokio::select! {
                Ok(()) = futures.join_all() => {

                }
                Ok(new_ips) = ip_changed.recv() => {
                    tracing::info!("Recieved Install change, restarting DNS server");
                    ips = new_ip;
                }
                else {
                    tracing::warn!("Futures aborted, shutting down install server");
                }
            }
            
        }
    }

    fn create_router(pool: SqlitePool) -> Router {
        let mut app = Router::new().fallback(fallback.into_service());

        app = app.merge(health::router());
        app = app.merge(setup::router(pool));

        //Only enable static content if we're in release mode
        #[cfg(not(debug_assertions))]
        {
            app = app.merge(crate::web::frontend::router());
        }

        app
    }

    async fn fallback() -> (StatusCode, String) {
        (
            StatusCode::NOT_FOUND,
            "404 - Yeah you're not finding what you want.".to_string(),
        )
    }
}

#[derive(Debug, Error)]
pub enum InstallEndpointsError {
    #[error(transparent)]
    CertManagerError(#[from] CertManagerError),

    #[error(transparent)]
    HyperError(#[from] hyper::Error),

    #[error(transparent)]
    JoinError(#[from] JoinError),

    #[error("Missing acme email")]
    MissingAcmeEmail,

    #[error(transparent)]
    RustlsError(#[from] rustls::Error),

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
}