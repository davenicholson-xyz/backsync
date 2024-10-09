use crate::{database::wallpaper, system::files};
use axum::{extract::Multipart, routing::post, Router};

pub async fn upload(multipart: Multipart) -> Result<String, String> {
    let data = files::upload_image(multipart).await.unwrap();
    wallpaper::add(data).await.unwrap();
    Ok(format!("aw is good"))
}

pub fn get_routes() -> Router {
    Router::new().route("/wallpaper/upload", post(upload))
}
