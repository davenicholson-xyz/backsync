use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};

use crate::system::config;

#[derive(Serialize, Deserialize, Default)]
pub struct SettingsResponse {
    wallhaven_apikey: String,
}

impl SettingsResponse {
    fn new() -> Self {
        Self {
            wallhaven_apikey: "".to_string(),
        }
    }
}

pub async fn settings() -> Json<SettingsResponse> {
    let mut settings = SettingsResponse::default();
    if let Some(wall_api) = config::get::<String>("wallheaven_apikey").unwrap() {
        settings.wallhaven_apikey = wall_api;
    }
    Json(settings)
}

pub fn get_routes() -> Router {
    Router::new().route("/settings", get(settings))
}
