use anyhow::anyhow;
use anyhow::Result;
use serde_json::from_slice;
//use std::fs;
use std::io::ErrorKind;
use std::io::Read;
use std::net::SocketAddr;
use std::thread;
//use std::time::Duration;
use std::{
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
};

use crate::commands;
//use crate::commands::commands::ClientCommand;
use crate::commands::commands::ServerCommand;
//use crate::commands::send_to_client;
//use rand::seq::IteratorRandom;

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

pub fn start(port: i32) -> Result<()> {
    let local_ip = local_ip_address::local_ip()?;
    let listener = TcpListener::bind(format! {"{}:{}", local_ip, port})?;
    let clients: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));

    let tcp_clients = clients.clone();
    thread::spawn(move || {
        let _ = handle_client(listener, tcp_clients);
    });

    //let loop_clients = clients.clone();
    //thread::spawn(move || loop {
    //    thread::sleep(Duration::from_secs(5));
    //
    //    let mut clients = loop_clients.lock().unwrap();
    //    if clients.len() > 0 {
    //        let mut rng = rand::thread_rng();
    //        let files = fs::read_dir("/Users/dave/projects/backsync/server/wallpaper").unwrap();
    //        let file = files.choose(&mut rng).unwrap().unwrap();
    //        let filepath = file.path();
    //        let filename = filepath.file_name().unwrap();
    //        let filename_os = filename.to_os_string();
    //        let file_str = filename_os.to_str().unwrap();
    //        let command = ClientCommand::SetWallpaper {
    //            id: String::from(file_str),
    //        };
    //        for client in clients.iter_mut() {
    //            send_to_client(client, &command).unwrap();
    //        }
    //    }
    //});
    Ok(())
}
