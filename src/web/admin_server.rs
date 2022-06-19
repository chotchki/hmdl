use std::{io, net::SocketAddr};

use axum::{
    body::{boxed, Full},
    handler::Handler,
    http::{header, StatusCode, Uri},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use rust_embed::RustEmbed;

use crate::GIT_VERSION;

pub struct AdminServer;

impl AdminServer {
    pub async fn create() -> io::Result<()> {
        // build our application with a route
        let app = Router::new()
            .route("/", get(index_handler))
            //Have to explictly add mappings for everything top level so the router doesn't have conflicts
            .route("/favicon.ico", static_handler.into_service())
            .route("/index.html", get(index_handler))
            .route("/manifest.json", static_handler.into_service())
            .route("/robots.txt", static_handler.into_service())
            .route("/icons/*file", static_handler.into_service())
            .route("/static/*file", static_handler.into_service())
            .fallback(get(not_found));

        let addr = SocketAddr::from(([0, 0, 0, 0], 80));

        tracing::info!("Web Server listening on {}", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();

        Ok(())
    }
}

// We use static route matchers ("/" and "/index.html") to serve our home
// page.
async fn index_handler() -> impl IntoResponse {
    static_handler("/index.html".parse::<Uri>().unwrap()).await
}

// We use a wildcard matcher ("/dist/*file") to match against everything
// within our defined assets directory. This is the directory on our Asset
// struct below, where folder = "examples/public/".
async fn static_handler(uri: Uri) -> impl IntoResponse {
    let mut path = uri.path().trim_start_matches('/').to_string();

    StaticFile(path)
}

// Finally, we use a fallback route for anything that didn't match.
async fn not_found() -> Html<&'static str> {
    Html("<h1>404</h1><p>Not Found</p>")
}

// Static Example from here: https://github.com/pyrossh/rust-embed/blob/master/examples/axum.rs
#[derive(RustEmbed)]
#[folder = "hearthstone-frontend/build"]
struct Asset;

pub struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> Response {
        let path = self.0.into();

        match Asset::get(path.as_str()) {
            Some(content) => {
                let body = boxed(Full::from(content.data));
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                Response::builder()
                    .header(header::CONTENT_TYPE, mime.as_ref())
                    .header("git-version", GIT_VERSION)
                    .body(body)
                    .unwrap()
            }
            None => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(boxed(Full::from("404")))
                .unwrap(),
        }
    }
}
