use std::{net::SocketAddr, sync::Arc};

use anyhow::Result;
use futures::{SinkExt, StreamExt};
use once_cell::sync::Lazy;
use serde::Serialize;
use tokio::{
    net::TcpListener,
    sync::{broadcast, Mutex},
};
use tokio_tungstenite::{accept_async, tungstenite::Message};

use crate::database;

#[derive(Serialize)]
pub struct ClientUpdate<T> {
    subject: String,
    clients: T,
}

#[derive(Serialize)]
pub struct ImageUpload {
    subject: String,
    progress: i32,
}

static BROADCAST_TX: Lazy<Arc<Mutex<broadcast::Sender<String>>>> = Lazy::new(|| {
    let (tx, _) = broadcast::channel::<String>(100);
    Arc::new(Mutex::new(tx))
});

pub async fn start() -> Result<()> {
    let addr = "127.0.0.1:3002".parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

    info!("Websocket listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let tx = BROADCAST_TX.clone();
        let mut rx = tx.lock().await.subscribe();

        tokio::spawn(async move {
            let ws_stream = accept_async(stream)
                .await
                .expect("Error during WebSocket handshake");
            client_update().await.unwrap();

            let (mut ws_sender, mut ws_receiver) = ws_stream.split();

            tokio::spawn(async move {
                while let Some(Ok(msg)) = ws_receiver.next().await {
                    if let Message::Text(text) = msg {
                        parse_message(text.clone()).await;
                    }
                }
            });

            while let Ok(message) = rx.recv().await {
                if ws_sender.send(Message::Text(message)).await.is_err() {
                    break; // Client disconnected
                }
            }
        });
    }

    Ok(())
}

async fn parse_message(msg: String) {
    println!("Received message: {}", msg);
}

pub async fn broadcast_message(message: String) {
    let tx = BROADCAST_TX.clone();
    let _ = tx.lock().await.send(message);
}

pub async fn upload_progress(prog: i32) -> Result<()> {
    let upload = ImageUpload {
        subject: "image_upload".to_string(),
        progress: prog,
    };
    let upload_string = serde_json::to_string(&upload)?;
    broadcast_message(upload_string).await;

    Ok(())
}

pub async fn client_update() -> Result<()> {
    let clients = database::clients::all().await?;
    let clients_update = ClientUpdate {
        subject: "clients_update".to_string(),
        clients: &clients,
    };
    let client_string = serde_json::to_string(&clients_update)?;
    broadcast_message(client_string).await;
    Ok(())
}
