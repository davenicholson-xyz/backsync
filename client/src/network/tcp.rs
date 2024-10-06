use anyhow::Result;
use std::io::Read;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

use crate::system;

pub fn connect_client(socket: SocketAddr) -> Result<()> {
    let server_stream = TcpStream::connect(socket)?;
    let local_listener = TcpListener::bind("127.0.0.1:0")?;

    let config_file = system::files::config_file()?;
    let local_port: SocketAddr = local_listener.local_addr()?;
    system::config::set(&config_file, "daemon_port", local_port.port())?;

    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    let _server_handle = create_tcp_connection(server_stream, tx.clone())?;
    let _local_handle = create_tcp_listener(local_listener, tx.clone())?;

    let rx_clone = Arc::clone(&rx);
    thread::spawn(move || {
        let _ = process_message(rx_clone);
    });

    loop {
        thread::park();
    }
}

fn handle_tcp_client(mut stream: TcpStream, sender: Sender<String>) {
    println!("TCP connected to {}", stream.peer_addr().unwrap());
    let mut buffer = [0; 1024];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client {} disconnected", stream.peer_addr().unwrap());
                break;
            }
            Ok(n) => {
                let message = String::from_utf8_lossy(&buffer[..n]);
                sender.send(message.into_owned()).unwrap();
            }
            Err(e) => {
                eprintln!("Error reading from client: {}", e);
                break;
            }
        }
    }
}

fn create_tcp_connection(stream: TcpStream, sender: Sender<String>) -> Result<JoinHandle<()>> {
    let handle = thread::spawn(move || {
        handle_tcp_client(stream, sender);
    });
    Ok(handle)
}

fn create_tcp_listener(listener: TcpListener, sender: Sender<String>) -> Result<JoinHandle<()>> {
    let handle = thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let sender = sender.clone();
                    thread::spawn(move || {
                        handle_tcp_client(stream, sender);
                    });
                }
                Err(e) => {
                    eprintln!("Error accepting local client connection: {}", e);
                }
            }
        }
    });

    Ok(handle)
}

fn process_message(rx: Arc<Mutex<mpsc::Receiver<String>>>) -> Result<()> {
    loop {
        let message = rx.lock().unwrap().recv().unwrap();
        println!("RCVD:{}", message);
    }
}
