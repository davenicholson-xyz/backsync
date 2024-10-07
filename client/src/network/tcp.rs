use anyhow::Result;
use std::io::{Read, Write};
use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;
use std::{
    net::{SocketAddr, TcpStream},
    thread,
};

use crate::commands::server::ServerCommand;
use crate::daemon::SERVER_SEND;

pub fn server_stream(
    socket: SocketAddr,
    sender_to_main: Sender<String>,
    receiver_from_main: Receiver<String>,
) -> Result<JoinHandle<()>> {
    let mut stream = TcpStream::connect(socket)?;

    let handshake = ServerCommand::Handshake;
    let serialized = serde_json::to_vec(&handshake)?;
    stream.write_all(&serialized)?;

    let handle = thread::spawn(move || {
        let mut buffer = [0; 1024];
        loop {
            match stream.read(&mut buffer) {
                Ok(0) => {
                    println!("connection dropped");
                    break;
                }
                Ok(bytes_read) => {
                    let message = String::from_utf8_lossy(&buffer[..bytes_read]);
                    sender_to_main.send(message.into_owned()).unwrap();
                }
                Err(e) => {
                    eprintln!("Error reading from client: {}", e);
                }
            }

            if let Ok(msg_to_send) = receiver_from_main.recv() {
                if let Err(err) = stream.write_all(msg_to_send.as_bytes()) {
                    println!("Failed to send message to server: {}", err);
                }
            }
        }
    });
    Ok(handle)
}

pub fn send_command_to_stream(command: ServerCommand) -> Result<(), String> {
    if let Some(global_sender) = SERVER_SEND.get() {
        let sender = global_sender.lock().unwrap();
        let serialized = serde_json::to_string(&command).unwrap();
        sender
            .send(serialized)
            .map_err(|e| format!("Failed to send message: {:?}", e))
    } else {
        Err("Global sender is not set".into())
    }
}
