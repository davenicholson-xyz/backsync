use axum::{
    extract::Query,
    routing::{get, post},
    Json, Router,
};
use hyper::StatusCode;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
struct SearchParams {
    url: String,
}

//async fn search(Query(params): Query<SearchParams>) -> Result<Json<Value>, StatusCode> {
//    dbg!(&params.url);
//    match reqwest::get(&params.url).await {
//        Ok(response) => {
//            // Try to parse the JSON from the response
//            match response.json::<Value>().await {
//                Ok(json) => Ok(Json(json)), // Return the JSON response
//                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR), // Handle JSON parsing error
//            }
//        }
//        Err(_) => Err(StatusCode::BAD_REQUEST), // Handle reqwest error
//    }
//}
async fn search(Json(params): Json<SearchParams>) -> Result<Json<Value>, StatusCode> {
    dbg!(&params.url); // Debug the received URL

    // Use the provided URL to make the request to the Wallhaven API
    match reqwest::get(&params.url).await {
        Ok(response) => {
            // Try to parse the JSON from the response
            match response.json::<Value>().await {
                Ok(json) => Ok(Json(json)), // Return the JSON response
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR), // Handle JSON parsing error
            }
        }
        Err(_) => Err(StatusCode::BAD_REQUEST), // Handle reqwest error
    }
}

pub fn get_routes() -> Router {
    Router::new().route("/wallhaven/search", post(search))
}
