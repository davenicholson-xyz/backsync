use anyhow::Result;
use commands::ClientCommand;

use crate::system::{
    files,
    wallpaper::{self, set_wallpaper},
};

pub mod commands;

pub fn handle(command: ClientCommand) -> Result<()> {
    info!("RCVD: {}", command);
    match command {
        ClientCommand::Handshake => {
            debug!("SERVER: 👋");
        }
        ClientCommand::SetWallpaper { id } => {
            debug!("SERVER sent wallaper SET request: {}", id);
            set_wallpaper(&id)?;
        }
        ClientCommand::SendWallpaper { id, data, set } => {
            files::save_wallpaper(id.clone(), data)?;
            if set {
                wallpaper::set_wallpaper(&id)?;
            }
        }
    }
    Ok(())
}
