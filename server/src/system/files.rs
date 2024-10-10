use std::ffi::OsStr;
use std::fs;
use std::fs::remove_file;
use std::fs::File;
use std::io::Cursor;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::anyhow;
use anyhow::Result;
use image::ImageFormat;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

use image::ImageReader;

use axum::extract::Multipart;

use crate::database;
use crate::database::wallpaper::Wallpaper;
use crate::system::files;
use crate::utils;
use homedir;

use crate::commands::send_to_client;
use crate::commands::ClientCommand;

use super::config;

pub trait PathBufExt {
    fn make_string(&self) -> String;
}

impl PathBufExt for PathBuf {
    fn make_string(&self) -> String {
        self.clone().into_os_string().into_string().unwrap()
    }
}

pub fn config_file() -> Result<String> {
    let crate_name = env!("CARGO_PKG_NAME");
    if let Some(mut home_dir) = homedir::my_home()? {
        home_dir.push(format!(".config/{}/config.toml", crate_name));
        Ok(home_dir.into_os_string().into_string().unwrap())
    } else {
        Err(anyhow!("Could not find users config directory"))
    }
}

#[allow(dead_code)]
pub fn cache_path() -> Result<PathBuf> {
    let crate_name = env!("CARGO_PKG_NAME");
    if let Some(mut home_dir) = homedir::my_home()? {
        home_dir.push(format!(".cache/{}", crate_name));
        fs::create_dir_all(&home_dir)?;
        Ok(home_dir)
    } else {
        Err(anyhow!("Could not find users cache directory"))
    }
}

#[allow(dead_code)]
pub fn config_path() -> Result<PathBuf> {
    let crate_name = env!("CARGO_PKG_NAME");
    if let Some(mut home_dir) = homedir::my_home()? {
        home_dir.push(format!(".config/{}", crate_name));
        Ok(home_dir)
    } else {
        Err(anyhow!("Could not find users cache directory"))
    }
}

pub fn ext_from_path(path: &str) -> Result<String> {
    Ok(Path::new(path)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap()
        .to_string())
}

pub async fn create_directory(path: &PathBuf) -> Result<()> {
    let _ = tokio::fs::create_dir_all(&path)
        .await
        .map_err(|e| anyhow!("failed to create wallpaper storage directory: {}", e));
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

        let file_ext = files::ext_from_path(&filename)?;

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

pub fn storage_path(additional_path: &str) -> PathBuf {
    let storage = config::get::<String>("storage").unwrap().unwrap();
    let mut storage_path = PathBuf::new();
    storage_path.push(storage);
    storage_path.push(additional_path);
    return storage_path;
}

pub async fn delete_wallpaper(id: &str, ext: &str) -> Result<()> {
    let wallpaper_path = storage_path(&format!("wallpaper/{}.{}", id, ext));
    let thumbnail_path = storage_path(&format!("wallpaper/.thumbs/{}.jpg", id));
    remove_file(wallpaper_path)?;
    remove_file(thumbnail_path)?;
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
