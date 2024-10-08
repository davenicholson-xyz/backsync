use crate::{database, system::files};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientCommand {
    Handshake,
    SetWallpaper {
        id: String,
    },
    SendWallpaper {
        id: String,
        data: Vec<u8>,
        set: bool,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerCommand {
    Handshake { hostname: String },
    RequestWallpaper { id: String },
}

impl fmt::Display for ClientCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientCommand::SendWallpaper { id, data, set } => {
                let _ = data;
                write!(
                    f,
                    "SendWallpaper {{ id: \"{}\", data: [DATA], set: {} }}",
                    id, set
                )
            }
            _ => {
                write!(f, "{:?}", self)
            }
        }
    }
}

impl fmt::Display for ServerCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

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
