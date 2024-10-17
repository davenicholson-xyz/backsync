use std::sync::Arc;

use crate::{
    commands::{self, command::Command},
    network::{
        self,
        tcp::{self, data::DataPacket},
    },
};
use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

static SEND_DATA: Lazy<Arc<Mutex<Option<Box<dyn Fn(Vec<u8>) + Send + Sync>>>>> =
    Lazy::new(|| Arc::new(Mutex::new(None)));

pub async fn spawn(server_port: i32) -> Result<()> {
    if let Some(udp) = network::udp::listen_for_broadcast(server_port)? {
        let (mut incoming_rx, send_data) = tcp::client::start(&udp.to_string()).await.unwrap();

        let mut send_data_guard = SEND_DATA.lock().await;
        *send_data_guard = Some(Box::new(move |data: Vec<u8>| {
            let _ = send_data(data);
        }));

        tokio::spawn(async move {
            while let Some(message) = incoming_rx.recv().await {
                let data = DataPacket::from_raw(message).unwrap();
                let command: Command = serde_json::from_slice(data.data()).unwrap();
                commands::handle(command).await.unwrap();
            }
        });
    } else {
        return Err(anyhow!("Failed to get UDP connection"));
    };

    // START CLIENT LISTENER HERE
    tokio::spawn(async move {});

    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for ctrl+c");

    Ok(())
}

pub async fn send_to_server(command: Command) -> Result<()> {
    info!("SENDING TO SERVER: {:?}", &command);
    let command_string = serde_json::to_string(&command)?;
    let data = DataPacket::from_str(&command_string);
    let send_data = SEND_DATA.lock().await;
    if let Some(ref send_data_fn) = *send_data {
        send_data_fn(data.to_raw());
        Ok(())
    } else {
        Err(anyhow!("send data function not initialising"))
    }
}
