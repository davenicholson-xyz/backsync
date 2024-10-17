use anyhow::Result;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

use crate::{
    commands::command::{Command, DaemonCommand},
    system::config,
};

use super::send_to_server;

pub async fn start() -> Result<()> {
    tokio::spawn(async move {
        let listener = TcpListener::bind("127.0.0.1:38040").await.unwrap();
        info!("Daemon listening on 127.0.0.1:38040");

        loop {
            let (mut socket, _) = listener.accept().await.unwrap();
            tokio::spawn(async move {
                let mut buffer = [0; 1024];
                match socket.read(&mut buffer).await {
                    Ok(0) => return,
                    Ok(n) => {
                        let raw = &buffer[..n];
                        let data = String::from_utf8_lossy(raw);
                        let rtn = run_command(&data).await;
                        socket.write_all(rtn.as_bytes()).await.unwrap();
                    }
                    Err(e) => {
                        eprintln!("error readinf from socket: {:?}", e);
                    }
                }
            });
        }
    });
    Ok(())
}

pub async fn run_command(cmd: &str) -> String {
    let command = serde_json::from_str::<DaemonCommand>(cmd).unwrap();
    match command {
        DaemonCommand::Lock => {
            let uuid = config::get::<String>("uuid").unwrap().unwrap();
            send_to_server(Command::ClientLock { uuid }).await.unwrap();
            return String::from("Wallpaper locked from server changes");
        }
        DaemonCommand::Unlock => {
            let uuid = config::get::<String>("uuid").unwrap().unwrap();
            send_to_server(Command::ClientUnlock { uuid })
                .await
                .unwrap();
            return String::from("Wallpaper unlocked and allows server changes");
        }
    };
}
