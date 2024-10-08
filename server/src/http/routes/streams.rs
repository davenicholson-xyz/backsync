use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};

use crate::database::{self, stream::Stream};

#[derive(Serialize, Deserialize)]
pub struct StreamResponse {
    streams: Vec<Stream>,
}

pub async fn fetch_all() -> Json<StreamResponse> {
    let streams = database::stream::all().await.unwrap();
    let response = StreamResponse { streams };
    Json(response)
}

pub fn get_routes() -> Router {
    Router::new().route("/streams/all", get(fetch_all))
}
