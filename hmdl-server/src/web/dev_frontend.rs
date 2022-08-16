use axum::{response::Response, routing::get, Extension, Router};
use hyper::{client::HttpConnector, Body, Request, Uri};

//Steal from here: https://github.com/tokio-rs/axum/blob/main/examples/reverse-proxy/src/main.rs
type Client = hyper::client::Client<HttpConnector, Body>;

const NPM_SERVICE: &str = "http://localhost:3000";

pub fn router() -> Router {
    let client = Client::new();

    Router::new()
        .route("/", get(index_handler))
        .route("/favicon.ico", get(handler))
        .route("/index.html", get(index_handler))
        .route("/manifest.json", get(handler))
        .route("/robots.txt", get(handler))
        .route("/icons/*file", get(handler))
        .route("/static/*file", get(handler))
        .layer(Extension(client))
}

async fn index_handler(
    Extension(client): Extension<Client>,
    // NOTE: Make sure to put the request extractor last because once the request
    // is extracted, extensions can't be extracted anymore.
    mut req: Request<Body>,
) -> Response<Body> {
    let uri = format!("{}{}", NPM_SERVICE, "/index.html");
    tracing::debug!("Got static content request for / requesting {}", uri);

    *req.uri_mut() = Uri::try_from(uri).unwrap();

    client.request(req).await.unwrap()
}

async fn handler(
    Extension(client): Extension<Client>,
    // NOTE: Make sure to put the request extractor last because once the request
    // is extracted, extensions can't be extracted anymore.
    mut req: Request<Body>,
) -> Response<Body> {
    let path = req.uri().path();

    let path_query = req
        .uri()
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or(path);

    let uri = format!("{}{}", NPM_SERVICE, path_query);
    tracing::debug!("Got static content request for {} requesting {}", path, uri);

    *req.uri_mut() = Uri::try_from(uri).unwrap();

    client.request(req).await.unwrap()
}
