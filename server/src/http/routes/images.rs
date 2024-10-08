use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ApiResponse {
    message: String,
}

pub async fn clone() -> Json<ApiResponse> {
    let response = ApiResponse {
        message: "Hello from the server".to_string(),
    };
    Json(response)
}

pub fn get_routes() -> Router {
    Router::new().route("/images/clone", get(clone))
}
