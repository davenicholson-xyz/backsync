use image::ImageReader;
use std::io::Cursor;
use std::path::PathBuf;

use axum::{extract::Multipart, routing::post, Router};

use crate::system::{config, files};
use crate::utils;

pub async fn upload(mut multipart: Multipart) -> Result<String, String> {
    let storage_dir = config::get::<String>("storage").unwrap().unwrap();

    let mut upload_dir = PathBuf::from(storage_dir);
    upload_dir.push("wallpaper");
    files::create_directory(&upload_dir).await.unwrap();

    let mut thumb_dir = upload_dir.clone();
    thumb_dir.push(".thumb");
    files::create_directory(&thumb_dir).await.unwrap();

    while let Some(mut field) = multipart
        .next_field()
        .await
        .map_err(|e| format!("failed to read file: {}", e))?
    {
        let filename = field
            .file_name()
            .map(|f| f.to_string())
            .unwrap_or_else(|| "default.png".to_string());

        let file_ext = files::ext_from_path(&filename).unwrap();

        let file_id = utils::seed(8);
        let file_path = upload_dir.join(format!("{}.{}", file_id, file_ext));
        let thumb_path = thumb_dir.join(format!("{}.jpg", file_id));

        let mut file_contents = Vec::new();

        while let Some(chunk) = field
            .chunk()
            .await
            .map_err(|e| format!("failed to read chunk: {}", e))?
        {
            file_contents.extend_from_slice(&chunk);
        }

        let img = ImageReader::new(Cursor::new(&file_contents))
            .with_guessed_format()
            .map_err(|e| format!("error detectign image format: {}", e))?
            .decode()
            .map_err(|e| format!("error decoding image format: {}", e))?;

        let thumbnail = img.thumbnail(300, 300);

        files::save_image(&img, &file_path).unwrap();
        files::save_image(&thumbnail, &thumb_path).unwrap();

        return Ok(format!("file saved to {:?}", file_path));
    }
    Err("no file".to_string())
}

pub fn get_routes() -> Router {
    Router::new().route("/wallpaper/upload", post(upload))
}
