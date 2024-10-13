use axum::{extract::Path, response::IntoResponse, routing::get, Json, Router};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

use crate::database::{self, clients::Client};

#[derive(Serialize, Deserialize)]
pub struct ClientResponse {
    streams: Vec<Client>,
}

#[derive(Serialize, Deserialize)]
pub struct ClientParams {
    addr: String,
}

pub async fn fetch_all() -> Json<ClientResponse> {
    let streams = database::clients::all().await.unwrap();
    let response = ClientResponse { streams };
    Json(response)
}

pub async fn fetch(Path(addr): Path<String>) -> impl IntoResponse {
    let response = database::clients::get(&addr).await;
    match response {
        Ok(stream) => Json(stream).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "Clienti not found").into_response(),
    }
}

pub fn get_routes() -> Router {
    Router::new()
        .route("/clients", get(fetch_all))
        .route("/clients/:addr", get(fetch))
    //.route("/streams/set/:wallpaper_id", get(send_wallpaper_to_all_streams))
    //.route("/streams/:addr/set/:wallpaper_id", get(send_wallpaper_to_stream))
}
