use axum::extract::DefaultBodyLimit;
use axum::response::IntoResponse;
use axum::routing::get_service;
use axum::Router;
use hyper::Request;
use hyper::StatusCode;
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir, ServeFile};

use super::routes;

pub async fn start(port: i32) {
    info!("HTTP server running: http://127.0.0.1:{}", port);
    http_server(port).await;
}

async fn handle_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "404 - not found")
}

pub async fn http_server(port: i32) {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_headers(Any);

    let app = Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .merge(routes::clients::get_routes())
        .merge(routes::wallpaper::get_routes())
        .merge(routes::wallhaven::get_routes())
        .merge(routes::system::get_routes())
        .route_service("/", ServeFile::new("static/index.html"))
        .fallback(handle_404)
        .layer(cors)
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024));

    let port = port as u16;
    let addr = format!("127.0.0.1:{}", port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
