use anyhow::anyhow;
use anyhow::Result;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::io::ErrorKind;
use std::net::IpAddr;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::sync::Mutex;

pub type ConnectionData = (SocketAddr, Vec<u8>);

#[derive(Debug, Clone)]
pub enum ClientEvent {
    Added(SocketAddr),
    Removed(SocketAddr),
}

pub static CLIENTS: Lazy<Arc<Mutex<HashMap<IpAddr, mpsc::Sender<Vec<u8>>>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

pub async fn start(
    port: i32,
) -> Result<(mpsc::Receiver<ConnectionData>, mpsc::Receiver<ClientEvent>)> {
    let local_ip = local_ip_address::local_ip()?;
    let listener = TcpListener::bind(format! {"{}:{}", local_ip, port}).await?;

    let (message_sender, message_receiver) = mpsc::channel::<ConnectionData>(10);
    let (clients_sender, clients_receiver) = mpsc::channel::<ClientEvent>(10);

    tokio::spawn(async move {
        accept_new_client(listener, message_sender, clients_sender)
            .await
            .unwrap();
    });

    Ok((message_receiver, clients_receiver))
}

pub async fn accept_new_client(
    listener: TcpListener,
    msg_sender: mpsc::Sender<ConnectionData>,
    client_sender: mpsc::Sender<ClientEvent>,
) -> Result<()> {
    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                let msg_sender_clone = msg_sender.clone();
                let client_sender_clone = client_sender.clone();
                tokio::spawn(async move {
                    if let Err(e) =
                        new_client_connection(stream, msg_sender_clone, client_sender_clone).await
                    {
                        eprintln!("Error handling client: {}: {:?}", addr, e);
                    }
                });
            }
            Err(e) => {
                return Err(anyhow!("Error accepting TCP connection: {:?}", e));
            }
        }
    }
}

async fn new_client_connection(
    stream: TcpStream,
    msg_sender: mpsc::Sender<ConnectionData>,
    client_sender: mpsc::Sender<ClientEvent>,
) -> Result<()> {
    let peer_address = stream.peer_addr()?;

    let (mut reader, mut writer) = tokio::io::split(stream);
    let (write_tx, mut write_rx) = mpsc::channel::<Vec<u8>>(10);

    {
        let mut client_writers = CLIENTS.lock().await;
        client_writers.insert(peer_address.ip(), write_tx.clone());
    }

    let _ = client_sender.send(ClientEvent::Added(peer_address)).await;

    let writer_task = tokio::spawn(async move {
        while let Some(data) = write_rx.recv().await {
            writer.write_all(&data).await?;
            writer.flush().await?;
        }
        Ok::<(), anyhow::Error>(())
    });

    let reader_task = tokio::spawn(async move {
        let mut buffer = [0; 1024];

        loop {
            match reader.read(&mut buffer).await {
                Ok(0) => {
                    remove_client(&peer_address).await?;
                    client_sender
                        .send(ClientEvent::Removed(peer_address))
                        .await?;
                    //{
                    //    let mut client_writers = CLIENTS.lock().await;
                    //    client_writers.remove(&peer_address.ip());
                    //}
                    break;
                }
                Ok(n) => {
                    receive_data(&peer_address, &buffer[..n], &msg_sender).await?;
                    if n == 0 {
                        break;
                    }
                }
                Err(e) => {
                    if e.kind() == ErrorKind::WouldBlock || e.kind() == ErrorKind::TimedOut {
                        continue;
                    }
                    remove_client(&peer_address).await?;
                    client_sender
                        .send(ClientEvent::Removed(peer_address))
                        .await?;
                    //{
                    //    let mut client_writers = CLIENTS.lock().await;
                    //    client_writers.remove(&peer_address.ip());
                    //}
                    eprintln!("Error reading from client {}: {:?}", peer_address, e);
                    break;
                } //Err(e) => {
            }
        }
        Ok::<(), anyhow::Error>(())
    });

    let _ = tokio::join!(writer_task, reader_task);

    Ok(())
}

async fn remove_client(peer_addr: &SocketAddr) -> Result<()> {
    let mut tcp_clients = CLIENTS.lock().await;
    tcp_clients.remove(&peer_addr.ip());
    Ok(())
}

pub async fn receive_data(
    peer_addr: &SocketAddr,
    data: &[u8],
    sender: &mpsc::Sender<ConnectionData>,
) -> Result<()> {
    let pa = peer_addr.clone();
    let _ = sender.send((pa, data.to_vec())).await?;
    Ok(())
}

#[allow(dead_code)]
pub async fn send_data(peer_addr: &SocketAddr, message: &[u8]) -> Result<()> {
    let clients = CLIENTS.lock().await;
    if let Some(write_tx) = clients.get(&peer_addr.ip()) {
        write_tx.send(message.to_vec()).await?;
        Ok(())
    } else {
        Err(anyhow!("client not connected"))
    }
}
