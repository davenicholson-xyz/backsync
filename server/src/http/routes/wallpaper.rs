use std::path::PathBuf;

use crate::database;
use crate::database::wallpaper::Wallpaper;
use crate::system::{config, files};
use axum::body::Bytes;
use axum::extract::Path;
use axum::response::{IntoResponse, Redirect};
use axum::routing::get;
use axum::Json;
use axum::{extract::Multipart, routing::post, Router};
use hyper::{header, StatusCode};
use serde::{Deserialize, Serialize};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[derive(Serialize, Deserialize)]
pub struct WallpapersResponse {
    wallpapers: Vec<Wallpaper>,
}

#[derive(Serialize, Deserialize)]
pub struct FetchThumbParams {
    id: String,
}

pub async fn upload(multipart: Multipart) -> Redirect {
    let data = files::upload_image(multipart).await.unwrap();
    database::wallpaper::add(data).await.unwrap();
    Redirect::to("/")
}

pub async fn fetch_all() -> Json<WallpapersResponse> {
    let wallpapers = database::wallpaper::all().await.unwrap();
    let response = WallpapersResponse { wallpapers };
    Json(response)
}

pub async fn fetch_thumbnail(Path(id): Path<String>) -> impl IntoResponse {
    // PUT THESE INTO A FUNCTION EACH
    let storage_dir = config::get::<String>("storage").unwrap().unwrap();
    let mut upload_dir = PathBuf::from(storage_dir);
    upload_dir.push("wallpaper");
    let mut thumb_dir = upload_dir.clone();
    thumb_dir.push(".thumb");
    let thumb_file = thumb_dir.join(format!("{}.jpg", id));

    match File::open(thumb_file).await {
        Ok(mut file) => {
            let mut buffer = Vec::new();
            if let Err(err) = file.read_to_end(&mut buffer).await {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to read file: {}", err),
                )
                    .into_response();
            }

            let response = (
                StatusCode::OK,
                [(header::CONTENT_TYPE, "image/jpeg")],
                Bytes::from(buffer),
            );
            response.into_response()
        }
        Err(_) => (StatusCode::NOT_FOUND, "Thumbnail not found".to_string()).into_response(),
    }
}

pub fn get_routes() -> Router {
    Router::new()
        .route("/wallpapers", get(fetch_all))
        .route("/wallpapers/upload", post(upload))
        .route("/wallpapers/thumbnail/:id", get(fetch_thumbnail))
}
