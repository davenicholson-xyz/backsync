use std::fs::File;
use std::io::Cursor;
use std::io::Read;
use std::sync::Arc;

use anyhow::anyhow;
use image::ImageFormat;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

use image::ImageReader;

use axum::extract::Multipart;

use crate::database::wallpaper::Wallpaper;
use crate::system::config;
use crate::system::files;
use crate::system::paths;
use crate::system::paths::storage_path;
use crate::utils;

use crate::commands::send_to_client;
use crate::commands::ClientCommand;
use anyhow::Result;

use crate::database;

pub async fn delete_wallpaper(id: &str, ext: &str) -> Result<()> {
    let wallpaper_path = paths::storage_path(&format!("wallpaper/{}.{}", id, ext));
    let thumbnail_path = paths::storage_path(&format!("wallpaper/.thumbs/{}.jpg", id));
    std::fs::remove_file(wallpaper_path)?;
    std::fs::remove_file(thumbnail_path)?;
    Ok(())
}

pub async fn send_wallpaper(id: String, stream: Arc<Mutex<TcpStream>>) -> Result<()> {
    let wallpaper = database::wallpaper::get(&id).await?;
    let storage = config::get::<String>("storage")?.unwrap();
    let filepath = format!(
        "{}/wallpaper/{}.{}",
        storage, wallpaper.id, wallpaper.extension
    );
    let mut file = File::open(filepath)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let command = ClientCommand::SendWallpaper {
        id,
        data: buffer,
        set: true,
    };
    send_to_client(stream, &command).await?;
    Ok(())
}

pub async fn upload_image(mut multipart: Multipart) -> Result<Wallpaper> {
    let upload_dir = storage_path("wallpaper");
    let thumb_dir = storage_path("wallpaper/.thumbs");

    files::create_directory(&upload_dir).await.unwrap();
    files::create_directory(&thumb_dir).await.unwrap();

    let file_id = utils::seed(8);

    while let Some(field) = multipart.next_field().await.unwrap() {
        let filename = field.file_name().unwrap().to_string();
        let data = field.bytes().await?;

        let file_ext = paths::ext_from_path(&filename)?;

        let file_path = upload_dir.join(format!("{}.{}", file_id, file_ext));
        let thumb_path = thumb_dir.join(format!("{}.jpg", file_id));

        let mut file_contents = Vec::new();

        file_contents.extend_from_slice(&data);
        let img = ImageReader::new(Cursor::new(&file_contents))
            .with_guessed_format()?
            .decode()?;

        let thumbnail = img.thumbnail(300, 300);

        img.save(&file_path)?;
        thumbnail
            .to_rgb8()
            .save_with_format(&thumb_path, ImageFormat::Jpeg)?;

        return Ok(Wallpaper {
            id: file_id,
            filename,
            extension: file_ext,
        });
    }

    Err(anyhow!("no file"))
}
