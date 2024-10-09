use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::Cursor;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::anyhow;
use anyhow::Result;
use image::DynamicImage;
use image::ImageFormat;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

use homedir;

use crate::commands::send_to_client;
use crate::commands::ClientCommand;

use super::config;

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

pub fn save_image(img: &DynamicImage, path: &PathBuf) -> Result<()> {
    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);

    let ext = match path.extension().and_then(|ext| ext.to_str()) {
        Some("png") => Ok(ImageFormat::Png),
        Some("jpeg") | Some("jpg") => Ok(ImageFormat::Jpeg),
        Some("gif") => Ok(ImageFormat::Gif),
        Some("bmp") => Ok(ImageFormat::Bmp),
        Some("ico") => Ok(ImageFormat::Ico),
        Some("tiff") => Ok(ImageFormat::Tiff),
        Some("webp") => Ok(ImageFormat::WebP),
        _ => Err("Unknown image format".to_string()),
    }
    .map_err(|e| anyhow!(e))?;

    img.write_to(&mut cursor, ext)
        .map_err(|e| anyhow!("Failed to encode thumbnail: {}", e))?;

    let mut file =
        File::create(&path).map_err(|e| anyhow!("Failed to create thumbnail file: {}", e))?;
    file.write_all(&buffer)
        .map_err(|e| anyhow!("Failed to write thumbnail to file: {}", e))?;

    Ok(())
}

pub async fn send_wallpaper(id: String, stream: Arc<Mutex<TcpStream>>) -> Result<()> {
    let storage = config::get::<String>("storage")?.unwrap();
    let filepath = format!("{}/{}", storage, id);
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
