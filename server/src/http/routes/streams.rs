use axum::{extract::Path, response::IntoResponse, routing::get, Json, Router};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

use crate::database::{self, stream::Stream};

#[derive(Serialize, Deserialize)]
pub struct StreamsResponse {
    streams: Vec<Stream>,
}

#[derive(Serialize, Deserialize)]
pub struct StreamParams {
    addr: String,
}

pub async fn fetch_all() -> Json<StreamsResponse> {
    let streams = database::stream::all().await.unwrap();
    let response = StreamsResponse { streams };
    Json(response)
}

pub async fn fetch(Path(addr): Path<String>) -> impl IntoResponse {
    let response = database::stream::get(&addr).await;
    match response {
        Ok(stream) => Json(stream).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "Stream not found").into_response(),
    }
}

pub fn get_routes() -> Router {
    Router::new()
        .route("/streams", get(fetch_all))
        .route("/streams/:addr", get(fetch))
}
