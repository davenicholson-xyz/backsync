use anyhow::Result;
use std::io::Read;
use std::net::{SocketAddr, TcpStream};
use std::thread;

use crate::commands;
use crate::commands::client::ClientCommand;

pub fn server_stream(socket: SocketAddr) -> Result<TcpStream> {
    let stream = TcpStream::connect(socket)?;
    info!("Connected to SERVER at {}", stream.peer_addr().unwrap());

    let mut stream_clone = stream.try_clone()?;

    thread::spawn(move || {
        let mut buffer = [0; 1024];
        loop {
            match stream_clone.read(&mut buffer) {
                Ok(0) => {
                    println!("connection closed");
                    break;
                }
                Ok(bytes_read) => {
                    let message = String::from_utf8_lossy(&buffer[..bytes_read]);
                    let command =
                        serde_json::from_str::<ClientCommand>(&message.to_string()).unwrap();
                    commands::handle(command).unwrap();
                }
                Err(e) => {
                    error!("Error reading server: {}", e);
                    break;
                }
            }
        }
    });

    Ok(stream)
}
