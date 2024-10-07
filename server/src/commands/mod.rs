pub mod client;
pub mod info;
pub mod server;

use anyhow::Result;
use client::ClientCommand;
use server::ServerCommand;
use std::{
    io::Write,
    net::TcpStream,
    sync::{Arc, Mutex},
};

pub fn handle(command: ServerCommand, stream: &mut TcpStream) -> Result<()> {
    info!("Received: ServerCommand::{}", command);
    match command {
        ServerCommand::Handshake => {
            let reply = ClientCommand::Handshake;
            send_to_client(stream, &reply)?;
        }
    }
    Ok(())
}

pub fn send_to_client(stream: &mut TcpStream, command: &ClientCommand) -> Result<()> {
    info!("Sending: ClientCommand::{}", command);
    let serialized = serde_json::to_vec(&command)?;
    stream.write_all(&serialized)?;
    stream.flush()?;
    Ok(())
}

#[allow(dead_code)]
pub fn send_to_all_client(
    clients: Arc<Mutex<Vec<TcpStream>>>,
    command: &ClientCommand,
) -> Result<()> {
    let mut clients = clients.lock().unwrap();
    for client in clients.iter_mut() {
        send_to_client(client, command)?;
    }
    Ok(())
}
