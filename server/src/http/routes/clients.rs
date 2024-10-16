use std::{collections::HashMap, net::IpAddr, str::FromStr};

use axum::{extract::Path, response::IntoResponse, routing::get, Json, Router};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
    commands::{command::Command, send_to_client},
    database::{self, clients::Client},
};

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
    let response = database::clients::get_by_addr(&addr).await;
    match response {
        Ok(stream) => Json(stream).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "Clienti not found").into_response(),
    }
}

pub async fn update(Path(params): Path<HashMap<String, String>>) -> impl IntoResponse {
    let uuid = params.get("uuid").unwrap();
    let field = params.get("field").unwrap();
    let value = params.get("value").unwrap();
    let _ = database::clients::update_field(&uuid, &field, &value).await;
    (StatusCode::OK, "this is it").into_response()
}

pub async fn delete(Path(uuid): Path<String>) -> impl IntoResponse {
    let _ = database::clients::delete(&uuid).await;
    (StatusCode::OK, "this is it").into_response()
}

pub async fn set_wallpaper(Path((id, code)): Path<(i32, String)>) -> impl IntoResponse {
    let wp = database::wallpaper::get(&code).await.unwrap();
    let filename = format!("{}.{}", wp.code, wp.extension);
    let client = database::clients::get_by_id(id).await.unwrap();
    if client.connected_at != "" {
        let ip = IpAddr::from_str(&client.addr).unwrap();
        let command = Command::SetWallpaper {
            filename: filename.clone(),
        };
        send_to_client(ip, &command).await.unwrap();
        database::clients::set_wallpaper(&ip.to_string(), &wp.code)
            .await
            .unwrap();
    }

    (StatusCode::OK, "this is it").into_response()
}

pub fn get_routes() -> Router {
    Router::new()
        .route("/clients", get(fetch_all))
        .route("/clients/:addr", get(fetch))
        .route("/clients/:id/set/:code", get(set_wallpaper))
        .route("/clients/:uuid/update/:field/:value", get(update))
        .route("/clients/:uuid/delete", get(delete))
}
