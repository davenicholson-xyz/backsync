use anyhow::Result;
use command::Command;

use crate::system::{
    files,
    wallpaper::{self, set_wallpaper},
};

pub mod command;

pub async fn handle(command: Command) -> Result<()> {
    info!("Received Command::{}", &command);
    match command {
        Command::SetWallpaper { filename } => {
            info!("SERVER sent wallaper SET request: {}", filename);
            set_wallpaper(&filename).await?;
        }
        Command::SendWallpaper {
            filename,
            data,
            set,
        } => {
            files::save_wallpaper(filename.clone(), data)?;
            if set {
                wallpaper::set_wallpaper(&filename).await?;
            }
        }
        _ => {}
    }
    Ok(())
}
