use crate::commands::command::Command;
use crate::commands::send_to_client;
use crate::database::wallpaper::Wallpaper;
use crate::system::{files, paths};
use crate::{database, utils};
use axum::body::Bytes;
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::routing::{delete, get};
use axum::Json;
use axum::{extract::Multipart, routing::post, Router};
use hyper::{header, StatusCode};
use paths::PathBufExt;
use serde::{Deserialize, Serialize};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[derive(Serialize, Deserialize)]
pub struct WallpapersResponse {
    wallpapers: Vec<Wallpaper>,
}

#[derive(Serialize, Deserialize)]
pub struct FetchThumbParams {
    code: String,
}

#[derive(serde::Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

pub async fn upload(multipart: Multipart) -> Json<Wallpaper> {
    let wallpaper = files::wallpaper::upload_image(multipart).await.unwrap();
    database::wallpaper::add(&wallpaper).await.unwrap();
    Json(wallpaper)
}

pub async fn fetch_all() -> Json<WallpapersResponse> {
    let wallpapers = database::wallpaper::all().await.unwrap();
    let response = WallpapersResponse { wallpapers };
    Json(response)
}

pub async fn fetch(
    Path(code): Path<String>,
) -> Result<Json<Wallpaper>, (StatusCode, Json<ErrorResponse>)> {
    if let Ok(wallpaper) = database::wallpaper::get(&code).await {
        Ok(Json(wallpaper))
    } else {
        let error_response = ErrorResponse {
            message: format!("Wallpaper not found: {}", code),
        };
        Err((StatusCode::NOT_FOUND, Json(error_response)))
    }
}

pub async fn fetch_thumbnail(Path(code): Path<String>) -> impl IntoResponse {
    let thumbs_dir = paths::storage_path("wallpaper/.thumbs").make_string();
    let thumb_file = format!("{}/{}.jpg", thumbs_dir, code);

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

pub async fn delete_wallpaper(Path(code): Path<String>) -> StatusCode {
    let (code, ext) = utils::split_filename(&code).unwrap();
    files::wallpaper::delete_wallpaper(&code, &ext)
        .await
        .unwrap();
    database::wallpaper::delete(&code).await.unwrap();
    StatusCode::OK
}

pub async fn set(Path(code): Path<String>) -> impl IntoResponse {
    let wp = database::wallpaper::get(&code).await.unwrap();
    let filename = format!("{}.{}", wp.code, wp.extension);
    let clients = database::clients::all_online().await.unwrap();
    for client in clients {
        //let ip = IpAddr::from_str(&client.addr).unwrap();
        let command = Command::SetWallpaper {
            filename: filename.clone(),
        };
        send_to_client(&client.uuid, &command).await.unwrap();
        database::clients::set_wallpaper(&client.uuid, &wp.code)
            .await
            .unwrap();
    }

    (StatusCode::OK, "this is it").into_response()
}

pub fn get_routes() -> Router {
    Router::new()
        .route("/wallpapers", get(fetch_all))
        .route("/wallpapers/upload", post(upload))
        .route("/wallpapers/code/:code", get(fetch))
        .route("/wallpapers/set/:filename", get(set))
        .route("/wallpapers/thumbnail/:code", get(fetch_thumbnail))
        .route("/wallpapers/delete/:code", delete(delete_wallpaper))
}
