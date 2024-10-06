use core::str;
use std::{
    net::{SocketAddr, UdpSocket},
    str::FromStr,
};

use anyhow::Result;

pub fn listen_for_broadcast(port: i32) -> Result<Option<SocketAddr>> {
    let udp_socket = UdpSocket::bind(format!("0.0.0.0:{}", port))?;
    let mut buf = [0; 1024];

    println!("waiting for udp broadcast");

    let (amt, src) = udp_socket.recv_from(&mut buf)?;
    let udp_message = str::from_utf8(&buf[..amt])?;
    println!("received {} from {}", udp_message, src);

    let prefix = "SVR::";
    if !udp_message.to_uppercase().starts_with(prefix) {
        return Ok(None);
    }

    let address_part = udp_message.trim_start_matches(prefix);

    if let Ok(addr) = SocketAddr::from_str(address_part) {
        return Ok(Some(addr));
    } else {
        return Ok(None);
    }
}
