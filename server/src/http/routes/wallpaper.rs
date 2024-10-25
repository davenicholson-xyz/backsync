use crate::commands::command::Command;
use crate::commands::send_to_client;
use crate::database::wallpaper::Wallpaper;
use crate::http::error::HttpError;
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
    pages: i64,
}

#[derive(Serialize, Deserialize)]
pub struct FetchThumbParams {
    code: String,
}

pub async fn upload(multipart: Multipart) -> Result<Json<Wallpaper>, HttpError> {
    let wallpaper = files::wallpaper::upload_image(multipart).await?;
    database::wallpaper::add(&wallpaper).await?;
    Ok(Json(wallpaper))
}

pub async fn fetch_all() -> Result<Json<WallpapersResponse>, HttpError> {
    let wallpapers = database::wallpaper::all().await?;
    let total_count = database::wallpaper::count().await? as f64;
    let pages = (total_count / 24.0).ceil() as i64;
    let response = WallpapersResponse { wallpapers, pages };
    Ok(Json(response))
}

pub async fn fetch(Path(code): Path<String>) -> Result<Json<Wallpaper>, HttpError> {
    let wallpaper = database::wallpaper::get(&code).await?;
    Ok(Json(wallpaper))
}

pub async fn page(Path(page): Path<u32>) -> Result<Json<WallpapersResponse>, HttpError> {
    let wallpapers = database::wallpaper::page(page).await?;
    let total_count = database::wallpaper::count().await? as f64;
    let pages = (total_count / 24.0).ceil() as i64;
    let response = WallpapersResponse { wallpapers, pages };
    Ok(Json(response))
}

pub async fn fetch_thumbnail(Path(code): Path<String>) -> Result<impl IntoResponse, HttpError> {
    let thumbs_dir = paths::storage_path("wallpaper/.thumbs").make_string();
    let thumb_file = format!("{}/{}.jpg", thumbs_dir, code);

    let mut file = File::open(thumb_file).await?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await?;
    let response = (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "image/jpeg")],
        Bytes::from(buffer),
    );
    Ok(response.into_response())
}

pub async fn delete_wallpaper(Path(code): Path<String>) -> Result<StatusCode, HttpError> {
    let (code, ext) = utils::split_filename(&code).unwrap();
    files::wallpaper::delete_wallpaper(&code, &ext).await?;
    database::wallpaper::delete(&code).await?;
    Ok(StatusCode::OK)
}

pub async fn set(Path(code): Path<String>) -> Result<StatusCode, HttpError> {
    let wp = database::wallpaper::get(&code).await?;
    let filename = format!("{}.{}", wp.code, wp.extension);
    let clients = database::clients::all_online().await?;
    for client in clients {
        let command = Command::SetWallpaper {
            filename: filename.clone(),
        };
        send_to_client(&client.uuid, &command).await?;
        database::clients::set_wallpaper(&client.uuid, &wp.code).await?;
    }

    Ok(StatusCode::OK)
}

pub fn get_routes() -> Router {
    Router::new()
        .route("/wallpapers", get(fetch_all))
        .route("/wallpapers/upload", post(upload))
        .route("/wallpapers/code/:code", get(fetch))
        .route("/wallpapers/page/:page", get(page))
        .route("/wallpapers/set/:filename", get(set))
        .route("/wallpapers/thumbnail/:code", get(fetch_thumbnail))
        .route("/wallpapers/delete/:code", delete(delete_wallpaper))
}
