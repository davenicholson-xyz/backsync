use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};

use crate::{http::server::HttpError, system::config};

#[derive(Serialize, Deserialize, Default)]
pub struct SettingsResponse {
    wallhaven_apikey: String,
}

#[allow(dead_code)]
impl SettingsResponse {
    fn new() -> Self {
        Self {
            wallhaven_apikey: "".to_string(),
        }
    }
}

pub async fn settings() -> Result<Json<SettingsResponse>, HttpError> {
    let mut settings = SettingsResponse::default();
    if let Some(wall_api) = config::get::<String>("wallhaven_apikey")? {
        settings.wallhaven_apikey = wall_api;
    }
    Ok(Json(settings))
}

pub fn get_routes() -> Router {
    Router::new().route("/settings", get(settings))
}
