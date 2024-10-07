use anyhow::Result;

use crate::{commands::commands::ServerCommand, daemon::send_to_server};

use super::files;

pub fn set_wallpaper(img: &str) -> Result<()> {
    info!("SETTING WP to {}", img);
    let mut cachepath = files::cache_path()?;
    cachepath.push(img);
    if !cachepath.exists() {
        info!("WP {} does not exist in cache. Requesting...", img);
        send_to_server(ServerCommand::RequestWallpaper {
            id: String::from(img),
        })?;
    } else {
        info!("WP {} in cache. Setting...", img);
        setwallpaper::from_file(&cachepath.into_os_string().into_string().unwrap())?;
    }
    Ok(())
}
