pub mod command;

use crate::network::tcp::data::DataPacket;
use crate::system::files;
use crate::{database, http, network};
use anyhow::Result;
use command::Command;
use std::net::SocketAddr;

pub async fn handle(command: Command) -> Result<()> {
    info!("Received: Command::{}", command);
    match command {
        Command::ClientInfo { uuid, ip, hostname } => {
            database::clients::insert(&uuid, &ip, &hostname).await?;
            http::websocket::client_update().await?;
            send_to_client(&uuid, &Command::Handshake).await?;
        }
        Command::RequestWallpaper { uuid, code } => {
            files::wallpaper::send_wallpaper(&code, &uuid).await?;
        }
        Command::ConfirmWallpaper { uuid, code } => {
            database::clients::set_wallpaper(&uuid, &code).await?;
            http::websocket::client_update().await?;
        }
        Command::ClientLock { uuid } => {
            println!("LOCKING {}", &uuid);
        }
        Command::ClientUnlock { uuid } => {
            println!("UNLOCKING {}", &uuid);
        }
        _ => {
            debug!("Command not implemented on the server: {}", command);
        }
    }
    Ok(())
}

pub async fn send_to_client(uuid: &str, command: &Command) -> Result<()> {
    let client = database::clients::get_by_uuid(uuid)
        .await
        .expect("could not find client");
    let socket: SocketAddr = format!("{}:0", client.addr)
        .parse()
        .expect("could not generate SocketAddr from client addr");
    let command_string = serde_json::to_string(&command)?;
    let data = DataPacket::from_str(&command_string);
    network::tcp::server::send_data(&socket, &data.to_raw()).await?;
    Ok(())
}
