use anyhow::anyhow;
use anyhow::Result;
use serde_json::from_slice;
use std::io::ErrorKind;
use std::io::Read;
use std::net::SocketAddr;
use std::thread;
use std::{
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
};

use crate::commands;
use crate::commands::commands::ServerCommand;

pub fn handle_client(listener: TcpListener, clients: Arc<Mutex<Vec<TcpStream>>>) -> Result<()> {
    let tcp_addr = listener.local_addr()?;
    info!("TCP listening on {}", tcp_addr);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                info!("CLIENT connected: {}", tcp_addr);
                let clients = Arc::clone(&clients);
                thread::spawn(move || {
                    let peer_addr = stream.peer_addr().unwrap();
                    client_connect(stream, clients).unwrap_or_else(|e| {
                        error!("Error handling client {}: {:?}", peer_addr, e);
                    })
                });
            }
            Err(e) => {
                return Err(anyhow!("Error accepting TCP connection: {:?}", e));
            }
        }
    }

    Ok(())
}

fn client_connect(mut stream: TcpStream, clients: Arc<Mutex<Vec<TcpStream>>>) -> Result<()> {
    let peer_address = stream.peer_addr()?;
    let mut buffer = [0; 1024];

    {
        let mut tcp_clients = clients.lock().unwrap();
        tcp_clients.push(stream.try_clone()?);
    }

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                info!("CLIENT disconnected: {}", peer_address);
                remove_client(&peer_address, &clients)?;
                break;
            }
            Ok(n) => {
                let message = match from_slice::<ServerCommand>(&buffer[..n]) {
                    Ok(msg) => msg,
                    Err(e) => {
                        error!(
                            "failed to deserialise message from {}: {:?}",
                            peer_address, e
                        );
                        continue;
                    }
                };

                if let Err(e) = commands::handle(message, &mut stream) {
                    error!("Error handling message from {}: {:?}", peer_address, e);
                }
            }
            Err(e) => {
                if e.kind() == ErrorKind::WouldBlock || e.kind() == ErrorKind::TimedOut {
                    continue;
                }
                error!("Error reading from client {}: {:?}", peer_address, e);
                remove_client(&peer_address, &clients)?;
                break;
            } //Err(e) => {
        }
    }

    Ok(())
}

fn remove_client(peer_addr: &SocketAddr, clients: &Arc<Mutex<Vec<TcpStream>>>) -> Result<()> {
    let mut tcp_clients = clients.lock().unwrap();
    tcp_clients.retain(|client| match client.peer_addr() {
        Ok(addr) if &addr == peer_addr => false,
        _ => true,
    });
    info!("Client removed: {}", peer_addr);
    Ok(())
}
