use std::collections::HashMap;

use axum::{extract::Path, response::IntoResponse, routing::get, Json, Router};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
    commands::{command::Command, send_to_client},
    database::{self, clients::Client},
    http::error::HttpError,
};

#[derive(Serialize, Deserialize)]
pub struct ClientResponse {
    streams: Vec<Client>,
}

#[derive(Serialize, Deserialize)]
pub struct ClientParams {
    addr: String,
}

pub async fn fetch_all() -> Result<Json<ClientResponse>, HttpError> {
    let streams = database::clients::all().await?;
    let response = ClientResponse { streams };
    Ok(Json(response))
}

pub async fn fetch(Path(addr): Path<String>) -> Result<Json<Client>, HttpError> {
    let client = database::clients::get_by_addr(&addr).await?;
    Ok(Json(client))
}

pub async fn update(
    Path(params): Path<HashMap<String, String>>,
) -> Result<impl IntoResponse, HttpError> {
    let uuid = params.get("uuid").unwrap();
    let field = params.get("field").unwrap();
    let value = params.get("value").unwrap();

    database::clients::update_field(&uuid, &field, &value).await?;
    Ok((StatusCode::OK, "this is it").into_response())
}

pub async fn delete(Path(uuid): Path<String>) -> Result<impl IntoResponse, HttpError> {
    database::clients::delete(&uuid).await?;
    Ok((StatusCode::OK, "this is it").into_response())
}

pub async fn set_wallpaper(
    Path((uuid, code)): Path<(String, String)>,
) -> Result<impl IntoResponse, HttpError> {
    let client = database::clients::get_by_uuid(&uuid).await?;
    if client.connected_at == "" {
        database::clients::update_field(&uuid, "syncwall", &code).await?;
        return Ok((StatusCode::OK, "this is it").into_response());
    }
    let wp = database::wallpaper::get(&code).await?;
    let filename = format!("{}.{}", wp.code, wp.extension);
    if client.connected_at != "" {
        let command = Command::SetWallpaper {
            filename: filename.clone(),
        };
        send_to_client(&uuid, &command).await?;
        database::clients::set_wallpaper(&uuid, &wp.code).await?;
    }

    Ok((StatusCode::OK, "this is it").into_response())
}

pub fn get_routes() -> Router {
    Router::new()
        .route("/clients", get(fetch_all))
        .route("/clients/:addr", get(fetch))
        .route("/clients/:id/set/:code", get(set_wallpaper))
        .route("/clients/:uuid/update/:field/:value", get(update))
        .route("/clients/:uuid/delete", get(delete))
}
