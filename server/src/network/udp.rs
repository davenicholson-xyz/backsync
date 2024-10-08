use std::net::{SocketAddr, UdpSocket};

use anyhow::Result;
use tokio::task;
use tokio::time::{sleep, Duration};

pub fn broadcast(port: i32) -> Result<()> {
    let local_ip = local_ip_address::local_ip()?;
    let broadcast_address: SocketAddr = format!("255.255.255.255:{}", port).parse()?;
    let msg = format!("SVR::{}:{}", local_ip, port);
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(true)?;
    info!("UDP: {}", msg);
    task::spawn(async move {
        loop {
            let _ = socket.send_to(msg.as_bytes(), broadcast_address);
            sleep(Duration::from_secs(5)).await;
        }
    });
    Ok(())
}
