use crate::database::wallpaper::Wallpaper;
use crate::system::files;
use crate::{database, utils};
use axum::body::Bytes;
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::routing::{delete, get};
use axum::Json;
use axum::{extract::Multipart, routing::post, Router};
use files::PathBufExt;
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

pub async fn upload(multipart: Multipart) -> Json<Wallpaper> {
    let wallpaper = files::upload_image(multipart).await.unwrap();
    database::wallpaper::add(&wallpaper).await.unwrap();
    Json(wallpaper)
}

pub async fn fetch_all() -> Json<WallpapersResponse> {
    let wallpapers = database::wallpaper::all().await.unwrap();
    let response = WallpapersResponse { wallpapers };
    Json(response)
}

pub async fn fetch_thumbnail(Path(id): Path<String>) -> impl IntoResponse {
    let thumbs_dir = files::storage_path("wallpaper/.thumbs").make_string();
    let thumb_file = format!("{}/{}.jpg", thumbs_dir, id);

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

pub async fn delete_wallpaper(Path(filename): Path<String>) -> StatusCode {
    let (id, ext) = utils::split_filename(&filename).unwrap();
    files::delete_wallpaper(&id, &ext).await.unwrap();
    database::wallpaper::delete(&id).await.unwrap();
    StatusCode::OK
}

pub fn get_routes() -> Router {
    Router::new()
        .route("/wallpapers", get(fetch_all))
        .route("/wallpapers/upload", post(upload))
        .route("/wallpapers/thumbnail/:id", get(fetch_thumbnail))
        .route("/wallpapers/delete/:filename", delete(delete_wallpaper))
}
