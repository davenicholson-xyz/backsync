use super::routes;
use anyhow::Result;
use axum::body::Body;
use axum::extract::DefaultBodyLimit;
use axum::middleware;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use hyper::header;
use hyper::Request;
use hyper::StatusCode;
use hyper::Uri;
use rust_embed::Embed;
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;

pub async fn start(port: i32) -> Result<()> {
    info!("HTTP server running: http://127.0.0.1:{}", port);
    http_server(port).await;
    Ok(())
}

async fn handle_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "404 - not found")
}

async fn log_request(req: Request<Body>, next: Next) -> Response {
    info!("{} {}", req.method(), req.uri().path());
    let response = next.run(req).await;
    info!("response status: {}", response.status());
    response
}

pub async fn http_server(port: i32) {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any);

    let app = Router::new()
        .route("/", get(index_handler))
        .merge(routes::clients::get_routes())
        .merge(routes::wallpaper::get_routes())
        .merge(routes::wallhaven::get_routes())
        .merge(routes::system::get_routes())
        .route("/index.html", get(index_handler))
        .route("/*file", get(static_handler))
        .fallback(handle_404)
        .layer(cors)
        .layer(middleware::from_fn(log_request))
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024));

    let port = port as u16;
    let addr = format!("127.0.0.1:{}", port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index_handler() -> impl IntoResponse {
    static_handler("/index.html".parse::<Uri>().unwrap()).await
}

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let mut path = uri.path().trim_start_matches('/').to_string();
    if path.starts_with("dist/") {
        path = path.replace("dist/", "");
    }
    StaticFile(path)
}

#[derive(Embed)]
#[folder = "web/build"]
struct Assset;

pub struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> Response {
        let path = self.0.into();
        match Assset::get(path.as_str()) {
            Some(content) => {
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
            }
            None => (StatusCode::NOT_FOUND, "404 not found").into_response(),
        }
    }
}
