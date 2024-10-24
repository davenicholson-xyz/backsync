use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};

use crate::{http::error::HttpError, system::config};

#[derive(Serialize, Deserialize, Default)]
pub struct SettingsResponse {
    wallhaven_apikey: String,
    wallhaven_username: String,
}

#[allow(dead_code)]
impl SettingsResponse {
    fn new() -> Self {
        Self {
            wallhaven_apikey: "".to_string(),
            wallhaven_username: "".to_string(),
        }
    }
}

pub async fn settings() -> Result<Json<SettingsResponse>, HttpError> {
    let mut settings = SettingsResponse::default();

    if let Some(wall_api) = config::get::<String>("wallhaven_apikey")? {
        settings.wallhaven_apikey = wall_api;
    }

    if let Some(wall_user) = config::get::<String>("wallhaven_username")? {
        settings.wallhaven_username = wall_user
    }

    Ok(Json(settings))
}

pub fn get_routes() -> Router {
    Router::new().route("/settings", get(settings))
}
