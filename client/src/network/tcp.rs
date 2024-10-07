use anyhow::Result;
use std::io::Read;
use std::net::{SocketAddr, TcpStream};
use std::thread;

use crate::commands;
use crate::commands::commands::ClientCommand;

pub fn server_stream(socket: SocketAddr) -> Result<TcpStream> {
    let stream = TcpStream::connect(socket)?;
    info!("Connected to SERVER at {}", stream.peer_addr().unwrap());

    let mut stream_clone = stream.try_clone()?;

    thread::spawn(move || {
        let mut length_prefix = [0; 4];
        loop {
            match stream_clone.read_exact(&mut length_prefix) {
                Ok(()) => {
                    let message_length = u32::from_be_bytes(length_prefix) as usize;
                    let mut buffer = vec![0; message_length];
                    match stream_clone.read_exact(&mut buffer) {
                        Ok(()) => {
                            let command = serde_json::from_slice::<ClientCommand>(&buffer).unwrap();
                            commands::handle(command).unwrap();
                        }
                        Err(e) => {
                            error!("Error reading server: {}", e);
                            break;
                        }
                    }
                }
                Err(e) => {
                    error!("Error reading length prefix: {}", e);
                    break;
                }
            }
        }
    });

    Ok(stream)
}
