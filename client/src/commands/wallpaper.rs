use crate::{commands, system::files};
use anyhow::Result;
use shared::server::{ServerCommand, ServerMessage};
use std::path::PathBuf;

pub fn set(filename: &str) -> Result<()> {
    let mut cached = PathBuf::from(files::cache_path()?);
    cached.push(filename);

    if cached.exists() {
        println!("{} exists. set as wallpaper", filename);
    } else {
        println!("{} does not exists. request from server", filename);
        let message = ServerMessage::new(ServerCommand::RequestWallpaper, filename);
        //commands::send_to_stream(message)?;
    }

    Ok(())
}
