use anyhow::{anyhow, Result};
use shared::client::ClientMessage;
use std::io::{ErrorKind, Read, Write};
use std::net::TcpStream;
use std::time::Duration;

pub fn send(mut stream: &TcpStream, message: &ClientMessage) -> Result<Option<String>> {
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    let serialized = serde_json::to_vec(&message).expect("Failed to serialize message");
    stream.write_all(&serialized)?;
    stream.flush()?;

    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(0) => Ok(None),
        Ok(bytes_read) => {
            let response = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
            Ok(Some(response))
        }
        Err(ref e) if e.kind() == ErrorKind::WouldBlock || e.kind() == ErrorKind::TimedOut => {
            Ok(None)
        }
        Err(e) => Err(anyhow!("Failed to read stream: {e}")),
    }
}
