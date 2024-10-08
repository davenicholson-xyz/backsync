use crate::commands::commands::ServerCommand;
use crate::network;
use crate::system;
use anyhow::{anyhow, Result};
use once_cell::sync::OnceCell;
use std::io::Write;
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub static SERVER_STREAM: OnceCell<Arc<Mutex<TcpStream>>> = OnceCell::new();

pub fn spawn(server_port: i32) -> Result<()> {
    if let Some(udp) = network::udp::listen_for_broadcast(server_port)? {
        system::config::set("server_ip", udp.ip())?;
        let stream = network::tcp::server_stream(udp)?;
        SERVER_STREAM.set(Arc::new(Mutex::new(stream))).unwrap();
        let hostname = gethostname::gethostname().into_string().unwrap();
        send_to_server(ServerCommand::Handshake { hostname })?;
    } else {
        return Err(anyhow!("Failed to get UDP connection"));
    };

    loop {
        thread::park();
    }
}
pub fn send_to_server(command: ServerCommand) -> Result<()> {
    if let Some(stream) = SERVER_STREAM.get() {
        let mut stream = stream.lock().unwrap();
        let message = serde_json::to_string(&command)?;
        stream.write_all(message.as_bytes())?;
    } else {
        return Err(anyhow!("Global stream could not be initialised"));
    }
    Ok(())
}
