use anyhow::Result;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
pub async fn start() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3002").await?;

    while let Ok((stream, _)) = listener.accept().await {
        // Spawn a new task for each connection
        tokio::spawn(async move {
            // Upgrade the TCP connection to a WebSocket connection asynchronously
            let ws_stream = accept_async(stream).await.unwrap();
            println!("New WebSocket connection established!");

            let (mut write, mut read) = ws_stream.split();

            // Echo incoming messages back to the client
            while let Some(Ok(msg)) = read.next().await {
                println!("Received message: {}", msg);

                if msg.is_text() || msg.is_binary() {
                    // Echo the message back to the client
                    write.send(msg).await.unwrap();
                }
            }
        });
    }

    //for stream in server.incoming() {
    //    tokio::spawn(async move {
    //        let stream = stream.unwrap();
    //        let mut websocket = accept(stream).unwrap();
    //        println!("new socket astablished");
    //        loop {
    //            let msg = websocket.read_message().unwrap();
    //            println!("received message: {}", msg);
    //
    //            if msg.is_text() || msg.is_binary() {
    //                websocket.write_message(msg).unwrap();
    //            }
    //        }
    //    });
    //}
    Ok(())
}
