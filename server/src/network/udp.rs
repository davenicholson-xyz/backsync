use std::net::{SocketAddr, UdpSocket};

use anyhow::Result;
use tokio::task;
use tokio::time::{sleep, Duration};

pub fn broadcst(broadcast_address: SocketAddr, msg: String) -> Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(true)?;
    task::spawn(async move {
        loop {
            let _ = socket.send_to(msg.as_bytes(), broadcast_address);
            println!("{}", msg);
            sleep(Duration::from_secs(5)).await;
        }
    });
    Ok(())
}
