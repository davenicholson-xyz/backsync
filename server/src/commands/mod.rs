pub mod info;

use anyhow::Result;
use std::{
    net::TcpStream,
    sync::{Arc, Mutex},
};

use shared::{client::ClientMessage, server::ServerMessage};

use crate::network;

pub fn handle(message: ServerMessage, stream: &mut TcpStream) -> Result<()> {
    dbg!(message);
    Ok(())
}

pub fn send_all(clients: &Arc<Mutex<Vec<TcpStream>>>, message: &ClientMessage) -> Result<()> {
    let tcp_clients = clients.lock().unwrap();
    for client in tcp_clients.iter() {
        network::messaging::send(client, &message)?;
    }
    Ok(())
}
