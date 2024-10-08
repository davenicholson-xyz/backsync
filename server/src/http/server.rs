use axum::{routing::get, Router};
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "static/"]
pub struct StaticFiles;

#[allow(dead_code)]
fn read_static(filename: &str) -> Option<Vec<u8>> {
    StaticFiles::get(filename).map(|file| file.data.to_vec())
}

pub async fn start(port: i32) {
    http_server(port).await;
}

pub async fn http_server(port: i32) {
    let app: Router<()> = Router::new().route("/", get(|| async { "hello world" }));
    let port = port as u16;
    let addr = format!("127.0.0.1:{}", port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
