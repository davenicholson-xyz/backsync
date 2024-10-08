pub mod commands;
pub mod info;

use anyhow::Result;
use commands::{ClientCommand, ServerCommand};
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

use crate::{database, system::files};

pub async fn handle(command: ServerCommand, stream: Arc<Mutex<TcpStream>>) -> Result<()> {
    info!("Received: ServerCommand::{}", command);
    match command {
        ServerCommand::Handshake { hostname } => {
            database::stream::add(&stream, &hostname).await?;
            let reply = ClientCommand::Handshake;
            send_to_client(stream, &reply).await?;
        }
        ServerCommand::RequestWallpaper { id } => {
            info!("CLIENT requested {}", id);
            files::send_wallpaper(id, stream).await?;
        }
    }
    Ok(())
}

pub async fn send_to_client(stream: Arc<Mutex<TcpStream>>, command: &ClientCommand) -> Result<()> {
    let serialized = serde_json::to_vec(&command)?;
    let length = (serialized.len() as u32).to_be_bytes();

    let mut stream = stream.lock().await;
    stream.write_all(&length).await?;
    stream.write_all(&serialized).await?;
    stream.flush().await?;

    Ok(())
}
