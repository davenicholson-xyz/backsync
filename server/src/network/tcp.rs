use crate::commands;
use crate::commands::commands::ServerCommand;
use crate::commands::info;
use crate::database;
use anyhow::anyhow;
use anyhow::Result;
use serde_json::from_slice;
use std::io::ErrorKind;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

pub async fn handle_client(
    listener: TcpListener,
    clients: Arc<Mutex<Vec<Arc<Mutex<TcpStream>>>>>,
) -> Result<()> {
    let tcp_addr = listener.local_addr()?;
    info!("TCP listening on {}", tcp_addr);

    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                info!("CLIENT connected: {}", addr);
                let clients = Arc::clone(&clients);
                let stream = Arc::new(Mutex::new(stream));
                tokio::spawn(async move {
                    if let Err(e) = client_connect(Arc::clone(&stream), clients).await {
                        error!("Error handling client: {}: {:?}", addr, e);
                    }
                });
            }
            Err(e) => {
                return Err(anyhow!("Error accepting TCP connection: {:?}", e));
            }
        }
    }
}

async fn client_connect(
    stream: Arc<Mutex<TcpStream>>,
    clients: Arc<Mutex<Vec<Arc<Mutex<TcpStream>>>>>,
) -> Result<()> {
    let peer_address = stream.lock().await.peer_addr()?;
    let mut buffer = [0; 1024];

    {
        let mut tcp_clients = clients.lock().await;
        tcp_clients.push(Arc::clone(&stream));
    }

    loop {
        let mut stream_guard = stream.lock().await;
        match stream_guard.read(&mut buffer).await {
            Ok(0) => {
                info!("CLIENT disconnected: {}", peer_address);
                drop(stream_guard);
                remove_client(&peer_address, &clients).await?;
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

                drop(stream_guard);

                if let Err(e) = commands::handle(message, Arc::clone(&stream)).await {
                    error!("Error handling message from {}: {:?}", peer_address, e);
                }
            }
            Err(e) => {
                if e.kind() == ErrorKind::WouldBlock || e.kind() == ErrorKind::TimedOut {
                    continue;
                }
                error!("Error reading from client {}: {:?}", peer_address, e);
                remove_client(&peer_address, &clients).await?;
                break;
            } //Err(e) => {
        }
    }

    Ok(())
}

async fn remove_client(
    peer_addr: &SocketAddr,
    clients: &Arc<Mutex<Vec<Arc<Mutex<TcpStream>>>>>,
) -> Result<()> {
    let mut tcp_clients = clients.lock().await;
    let mut i = 0;
    while i < tcp_clients.len() {
        let client = &tcp_clients[i];
        let client_addr = client.lock().await.peer_addr()?;
        if &client_addr == peer_addr {
            tcp_clients.remove(i);
        } else {
            i += 1;
        }
    }
    database::stream::remove(&peer_addr).await?;
    Ok(())
}

pub async fn start(port: i32) -> Result<()> {
    let local_ip = local_ip_address::local_ip()?;
    let listener = TcpListener::bind(format! {"{}:{}", local_ip, port}).await?;
    let clients: Arc<Mutex<Vec<Arc<Mutex<TcpStream>>>>> = Arc::new(Mutex::new(Vec::new()));

    let tcp_clients = clients.clone();
    handle_client(listener, tcp_clients).await?;
    Ok(())
}
