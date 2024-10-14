use std::{net::SocketAddr, sync::Arc};

use anyhow::Result;
use futures::{SinkExt, StreamExt};
use tokio::{net::TcpListener, sync::broadcast};
use tokio_tungstenite::{accept_async, tungstenite::Message};

pub async fn start() -> Result<()> {
    let addr = "127.0.0.1:3002".parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

    // Create a broadcast channel
    let (tx, _) = broadcast::channel::<String>(100);
    let tx = Arc::new(tx); // Arc to share across tasks

    println!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let tx = tx.clone();
        let mut rx = tx.subscribe(); // Each client gets its own receiver

        tokio::spawn(async move {
            let ws_stream = accept_async(stream)
                .await
                .expect("Error during WebSocket handshake");
            println!("New WebSocket connection established");

            // Split the WebSocket into sender and receiver
            let (mut ws_sender, mut ws_receiver) = ws_stream.split();

            // Spawn a task to handle incoming messages from the client
            let tx_inner = tx.clone();
            tokio::spawn(async move {
                while let Some(Ok(msg)) = ws_receiver.next().await {
                    if let Message::Text(text) = msg {
                        // Parse the incoming message
                        parse_message(text.clone()).await;
                        // Broadcast the message to all clients
                        let _ = tx_inner.send(text.clone());
                    }
                }
            });

            // Continuously send broadcast messages to the client
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
    // Parse and handle the message, e.g., parse JSON or commands
    println!("Received message: {}", msg);
}

async fn broadcast_message(tx: Arc<broadcast::Sender<String>>, message: String) {
    let _ = tx.send(message);
}
