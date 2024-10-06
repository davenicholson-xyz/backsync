pub mod info;
pub mod wallpaper;

use crate::{network, system};
use anyhow::Result;
use shared::{
    client::{ClientCommand, ClientMessage},
    server::ServerMessage,
};

pub fn send_daemon(message: ClientMessage) -> Result<Option<String>> {
    let port: Option<i32> = system::config::get("daemon_port")?;
    let local_tcp = format!("127.0.0.1:{}", port.unwrap());
    let response = network::messaging::send(&local_tcp, message)?;
    Ok(response)
}

pub fn send_server(message: ServerMessage) -> Result<Option<String>> {
    let server_address: Option<String> = system::config::get("server_address")?;
    let response = network::messaging::send(&server_address.unwrap(), message)?;
    Ok(response)
}

pub fn handle(message: ClientMessage) -> Result<()> {
    match message.command {
        ClientCommand::Test => {
            println!(
                "The test commend to the daemon received: {}",
                message.content
            );
        }
        ClientCommand::Handshake => {}
        ClientCommand::SetWallpaper => {
            wallpaper::set(&message.content)?;
        }
        ClientCommand::ImageData => {
            println!("image data not sent during a request");
        }
    }
    Ok(())
}
