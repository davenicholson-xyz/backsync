pub mod tcp;
pub mod udp;

use crate::commands;
use crate::{commands::command::Command, database, network};
use anyhow::Result;
use network::tcp::server::ClientEvent;
use tcp::data::DataPacket;

pub async fn start_tcp(network_port: i32) -> Result<()> {
    let (mut msg_tx, mut client_rx) = network::tcp::server::start(network_port).await?;
    let local_ip = local_ip_address::local_ip()?;
    info!("TCP listening on {}:{}", local_ip, network_port);

    tokio::spawn(async move {
        while let Some((_, data)) = msg_tx.recv().await {
            if !data.is_empty() {
                let data = DataPacket::from_raw(data).unwrap();
                let command: Command = serde_json::from_slice(data.data()).unwrap();
                commands::handle(command).await.unwrap();
            }
        }
    });

    tokio::spawn(async move {
        while let Some(client_event) = client_rx.recv().await {
            match client_event {
                ClientEvent::Added(_addr) => {
                    //commands::send_to_client(addr.ip(), &Command::Handshake)
                    //.await
                    //.unwrap();
                }
                ClientEvent::Removed(addr) => {
                    database::clients::disconnected_by_ip(addr.ip())
                        .await
                        .unwrap();
                }
            }
        }
    });
    Ok(())
}
