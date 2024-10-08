use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use hyper::Uri;
use include_dir::{include_dir, Dir};

static STATIC_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/static");

pub async fn start(port: i32) {
    http_server(port).await;
}

pub async fn http_server(port: i32) {
    let app: Router<()> = Router::new()
        .route("/", get(serve_index))
        .route("/static/*file", get(serve_static_file));
    let port = port as u16;
    let addr = format!("127.0.0.1:{}", port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn serve_index() -> impl IntoResponse {
    if let Some(file) = STATIC_DIR.get_file("index.html") {
        let mime = mime_guess::from_path("index.html").first_or_octet_stream();
        Response::builder()
            .header(axum::http::header::CONTENT_TYPE, mime.as_ref())
            .body(Body::from(file.contents().to_vec()))
            .unwrap()
    } else {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("404 index not found"))
            .unwrap()
    }
}

async fn serve_static_file(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches("/static/");

    if let Some(file) = STATIC_DIR.get_file(path) {
        let mime = mime_guess::from_path(path).first_or_octet_stream();
        info!("serving {}", path);
        Response::builder()
            .header(axum::http::header::CONTENT_TYPE, mime.as_ref())
            .body(Body::from(file.contents().to_vec()))
            .unwrap()
    } else {
        info!("404");
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("404 not found"))
            .unwrap()
    }
}
