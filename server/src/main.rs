mod flags;
mod network;

use std::{net::SocketAddr, time::Duration};

use anyhow::Result;
#[tokio::main]
async fn main() -> Result<()> {
    let flags = flags::cli_args();

    let local_ip = local_ip_address::local_ip()?;
    let port = flags.port.unwrap_or(37878);
    let broadcast_address: SocketAddr = format!("255.255.255.255:{}", port).parse()?;
    let msg = format!("JOIN::{}:{}", local_ip, port);

    network::udp::broadcst(broadcast_address, msg)?;

    loop {
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}
