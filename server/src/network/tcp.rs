use anyhow::anyhow;
use anyhow::Result;
use std::io::{Read, Write};
use std::net::SocketAddr;
use std::thread;
use std::{
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
};

pub fn handle_client(listener: TcpListener, clients: Arc<Mutex<Vec<TcpStream>>>) -> Result<()> {
    let tcp_addr = listener.local_addr()?;
    println!("tcp listening on {}", tcp_addr);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("ner client connected from {}", tcp_addr);
                let clients = Arc::clone(&clients);
                thread::spawn(move || {
                    let peer_addr = stream.peer_addr().unwrap();
                    client_connect(stream, clients).unwrap_or_else(|e| {
                        println!("Error handling client {}: {:?}", peer_addr, e);
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
                println!("Client disconnected: {}", peer_address);
                remove_client(&peer_address, &clients)?;
                break;
            }
            Ok(n) => {
                println!("Received {} bytes from {}", n, peer_address);
                stream.write_all(&buffer[..n])?;
            }
            Err(e) => {
                println!("Error reading from client {}: {:?}", peer_address, e);
                remove_client(&peer_address, &clients)?;
                break;
            }
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
    println!("Client removed: {}", peer_addr);
    Ok(())
}

pub fn send_message(
    clients: Arc<Mutex<Vec<TcpStream>>>,
    target: &SocketAddr,
    message: &str,
) -> Result<()> {
    let tcp_clients = clients.lock().unwrap();

    for mut client in tcp_clients.iter() {
        if let Ok(peer_addr) = client.peer_addr() {
            if &peer_addr == target {
                client.write_all(message.as_bytes())?;
                println!("Send message to {}, {}", peer_addr, message);
                return Ok(());
            }
        }
    }
    Ok(())
}

pub fn send_message_to_all_clients(
    clients: Arc<Mutex<Vec<TcpStream>>>,
    message: &str,
) -> Result<()> {
    let tcp_clients = clients.lock().unwrap();

    for mut client in tcp_clients.iter() {
        if let Err(e) = client.write_all(message.as_bytes()) {
            println!("Error sending message to {}: {:?}", client.peer_addr()?, e);
        } else {
            println!("Send {} to {}", message, client.peer_addr().unwrap());
        }
    }

    Ok(())
}
