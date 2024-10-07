use crate::network;
use crate::system;
use anyhow::{anyhow, Result};
use once_cell::sync::OnceCell;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::{sync::mpsc, thread};

pub static SERVER_SEND: OnceCell<Arc<Mutex<Sender<String>>>> = OnceCell::new();

pub fn spawn(server_port: i32) -> Result<()> {
    if let Some(udp) = network::udp::listen_for_broadcast(server_port)? {
        system::config::set("server_ip", udp.ip())?;

        let (tx_stream_to_main, rx_stream_to_main) = mpsc::channel();
        let (tx_main_to_stream, rx_main_to_stream) = mpsc::channel();

        SERVER_SEND
            .set(Arc::new(Mutex::new(tx_main_to_stream)))
            .unwrap();

        let _handle = network::tcp::server_stream(udp, tx_stream_to_main, rx_main_to_stream)?;

        thread::spawn(move || {
            while let Ok(receieved_message) = rx_stream_to_main.recv() {
                println!("Received from TCP: {}", receieved_message);
            }
        });
    } else {
        return Err(anyhow!("Failed to get UDP connection"));
    };

    loop {
        thread::park();
    }
}
