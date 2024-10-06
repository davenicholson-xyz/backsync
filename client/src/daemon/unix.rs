use anyhow::anyhow;
use anyhow::Result;

use crate::network;
use crate::system;

pub fn spawn(port: i32) -> Result<()> {
    if let Some(addr) = network::udp::listen_for_broadcast(port)? {
        system::config::set("server_address", addr.to_string())?;
        network::tcp::connect_client(addr)?;
    } else {
        return Err(anyhow!("failed to get UDP"));
    }
    Ok(())
}
