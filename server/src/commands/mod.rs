pub mod command;

use crate::network::tcp::data::DataPacket;
use crate::system::files;
use crate::{database, network};
use anyhow::Result;
use command::Command;
use std::net::{IpAddr, SocketAddr};

pub async fn handle(command: Command, ip: IpAddr) -> Result<()> {
    info!("Received: Command::{}", command);
    match command {
        Command::Handshake => {
            let cmd = Command::Handshake;
            send_to_client(ip, &cmd).await?;
        }
        Command::ClientInfo { ip, hostname } => {
            database::clients::insert(&ip, &hostname).await?;
        }
        Command::RequestWallpaper { code } => {
            debug!("CLIENT requested wallpaperi {}", code);
            files::wallpaper::send_wallpaper(code, ip).await?;
        }
        _ => {
            debug!("Command not implemented on the server: {}", command);
        }
    }
    Ok(())
}

pub async fn send_to_client(ip: IpAddr, command: &Command) -> Result<()> {
    let command_string = serde_json::to_string(&command)?;
    let data = DataPacket::from_str(&command_string);
    let socket = SocketAddr::new(ip, 0);
    network::tcp::server::send_data(&socket, &data.to_raw()).await?;
    Ok(())
}
