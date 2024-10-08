pub mod commands;
pub mod info;

use anyhow::Result;
use commands::{ClientCommand, ServerCommand};
use std::{io::Write, net::TcpStream};

use crate::system::files;

pub fn handle(command: ServerCommand, stream: &mut TcpStream) -> Result<()> {
    info!("Received: ServerCommand::{}", command);
    match command {
        ServerCommand::Handshake => {
            let reply = ClientCommand::Handshake;
            send_to_client(stream, &reply)?;
        }
        ServerCommand::RequestWallpaper { id } => {
            info!("CLIENT requested {}", id);
            files::send_wallpaper(id, stream)?;
        }
    }
    Ok(())
}

pub fn send_to_client(stream: &mut TcpStream, command: &ClientCommand) -> Result<()> {
    let serialized = serde_json::to_vec(&command)?;
    let length = (serialized.len() as u32).to_be_bytes();

    stream.write_all(&length)?;
    stream.write_all(&serialized)?;
    stream.flush()?;
    Ok(())
}

//#[allow(dead_code)]
//pub fn send_to_all_client(
//    clients: Arc<Mutex<Vec<TcpStream>>>,
//    command: &ClientCommand,
//) -> Result<()> {
//    let mut clients = clients.lock().unwrap();
//    for client in clients.iter_mut() {
//        send_to_client(client, command)?;
//    }
//    Ok(())
//}
