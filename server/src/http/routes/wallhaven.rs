use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    database::{self, wallpaper::Wallpaper},
    http::{self, server::HttpError},
    system::files,
};

#[derive(Serialize, Deserialize)]
struct SearchParams {
    url: String,
}

#[derive(Serialize, Deserialize)]
struct UploadParams {
    url: String,
}

async fn search(Json(params): Json<SearchParams>) -> Result<Json<Value>, HttpError> {
    let response = reqwest::get(&params.url).await?;
    let json = response.json::<Value>().await?;
    Ok(Json(json))
}

async fn upload(Json(params): Json<UploadParams>) -> Result<Json<Wallpaper>, HttpError> {
    let url = &params.url;
    if let Ok(wp) = database::wallpaper::get_by_origin(&url).await {
        return Ok(Json(wp));
    } else {
        http::websocket::upload_progress(10).await?;
        let mut wallpaper = files::wallpaper::save_image_from_url(url).await?;
        wallpaper.origin = url.clone();
        database::wallpaper::add(&wallpaper).await?;
        http::websocket::upload_progress(100).await?;
        return Ok(Json(wallpaper));
    }
}

pub fn get_routes() -> Router {
    Router::new()
        .route("/wallhaven/upload", post(upload))
        .route("/wallhaven/search", post(search))
}
