use axum::{extract::Multipart, routing::post, Router};

pub async fn upload(mut multipart: Multipart) -> Result<String, String> {
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| format!("failed to read file: {}", e))?
    {
        let filename = field
            .file_name()
            .map(|f| f.to_string())
            .unwrap_or_else(|| "default.png".to_string());
        dbg!(&filename);
        return Ok(format!("got filename: {}", &filename));
    }
    Err("no file".to_string())
}

pub fn get_routes() -> Router {
    Router::new().route("/wallpaper/upload", post(upload))
}
