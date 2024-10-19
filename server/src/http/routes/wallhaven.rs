use axum::{routing::post, Json, Router};
use hyper::StatusCode;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    database::{self, wallpaper::Wallpaper},
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

async fn search(Json(params): Json<SearchParams>) -> Result<Json<Value>, StatusCode> {
    match reqwest::get(&params.url).await {
        Ok(response) => match response.json::<Value>().await {
            Ok(json) => Ok(Json(json)),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

async fn upload(Json(params): Json<UploadParams>) -> Result<Json<Wallpaper>, StatusCode> {
    let url = &params.url;
    dbg!(&url);
    if let Ok(wp) = database::wallpaper::get_by_origin(&url).await {
        return Ok(Json(wp));
    } else {
        let mut wallpaper = files::wallpaper::save_image_from_url(url).await.unwrap();
        wallpaper.origin = url.clone();
        database::wallpaper::add(&wallpaper).await.unwrap();
        return Ok(Json(wallpaper));
    }
}

pub fn get_routes() -> Router {
    Router::new()
        .route("/wallhaven/upload", post(upload))
        .route("/wallhaven/search", post(search))
}
