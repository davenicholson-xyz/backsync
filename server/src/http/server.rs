use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

use super::routes;

pub async fn start(port: i32) {
    info!("HTTP server running: http://127.0.0.1:{}", port);
    http_server(port).await;
}

pub async fn http_server(port: i32) {
    let app = Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .route_service("/", ServeFile::new("static/index.html"))
        .merge(routes::streams::get_routes());

    let port = port as u16;
    let addr = format!("127.0.0.1:{}", port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
