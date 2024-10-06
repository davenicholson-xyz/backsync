use std::net::{SocketAddr, UdpSocket};

use anyhow::Result;
use tokio::task;
use tokio::time::{sleep, Duration};

pub fn broadcast(broadcast_address: SocketAddr, msg: String) -> Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(true)?;
    println!("SERVER BCAST -> {}", msg);
    task::spawn(async move {
        loop {
            let _ = socket.send_to(msg.as_bytes(), broadcast_address);
            sleep(Duration::from_secs(5)).await;
        }
    });
    Ok(())
}
