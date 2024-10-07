pub mod client;
pub mod info;
pub mod server;

use anyhow::Result;
use client::ClientCommand;
use server::ServerCommand;
use std::{io::Write, net::TcpStream};

pub fn handle(command: ServerCommand, stream: &mut TcpStream) -> Result<()> {
    match command {
        ServerCommand::Handshake => {
            println!("received handshake from client");
            send_to_client(stream, ClientCommand::Handshake)?;
        }
    }
    Ok(())
}

pub fn send_to_client(stream: &mut TcpStream, command: ClientCommand) -> Result<()> {
    let serialized = serde_json::to_vec(&command)?;
    stream.write_all(&serialized)?;
    stream.flush()?;
    Ok(())
}
