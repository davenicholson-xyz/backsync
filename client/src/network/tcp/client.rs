use std::net::SocketAddr;

use anyhow::Result;
use tokio::{
    io::{split, AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::mpsc,
};

pub async fn start(
    server_addr: &str,
) -> Result<(mpsc::Receiver<Vec<u8>>, impl Fn(Vec<u8>) -> Result<()>)> {
    let server_addr: SocketAddr = server_addr.parse()?;
    let stream = TcpStream::connect(server_addr).await?;

    let (mut reader, mut writer) = split(stream);

    let (incoming_tx, incoming_rx) = mpsc::channel::<Vec<u8>>(10);
    let (write_tx, mut write_rx) = mpsc::channel::<Vec<u8>>(10);

    tokio::spawn(async move {
        let mut buffer = [0; 1024];
        loop {
            match reader.read(&mut buffer).await {
                Ok(0) => {
                    println!("connection closed by server");
                    break;
                }
                Ok(n) => {
                    incoming_tx.send(buffer[..n].to_vec()).await?;
                }
                Err(e) => {
                    eprintln!("error reading from server: {:?}", e);
                    break;
                }
            }
        }

        Ok::<(), anyhow::Error>(())
    });

    tokio::spawn(async move {
        while let Some(data) = write_rx.recv().await {
            writer.write_all(&data).await?;
            writer.flush().await?;
        }
        Ok::<(), anyhow::Error>(())
    });

    let send_data = move |message: Vec<u8>| -> Result<()> {
        write_tx.try_send(message)?;
        Ok(())
    };

    Ok((incoming_rx, send_data))
}
