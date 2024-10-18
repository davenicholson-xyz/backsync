use axum::{routing::post, Json, Router};
use hyper::StatusCode;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
struct SearchParams {
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

pub fn get_routes() -> Router {
    Router::new().route("/wallhaven/search", post(search))
}
