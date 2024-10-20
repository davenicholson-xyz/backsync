use anyhow::Result;
use commands::command::DaemonCommand;
use flags::Action;

pub mod commands;
pub mod daemon;
mod flags;
mod network;
mod system;

mod utils;

#[macro_use]
extern crate log;
extern crate simplelog;

use system::config;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

#[tokio::main]
async fn main() -> Result<()> {
    let flags = flags::cli_args();
    let crate_name = env!("CARGO_PKG_NAME");

    system::logger::start(&format!("{}.log", crate_name));

    match &flags.command {
        Some(Action::INIT { port }) => {
            let server_port = config::flag_file_default(*port, "port", 37878)?;
            daemon::listener::start().await?;
            daemon::spawn(server_port).await?;
        }
        Some(Action::WALLPAPER { lock, unlock }) => {
            if lock.to_owned() {
                command_to_daemon(&DaemonCommand::Lock).await?;
                return Ok(());
            }

            if unlock.to_owned() {
                command_to_daemon(&DaemonCommand::Unlock).await?;
                return Ok(());
            }
        }
        Some(Action::STOP) => {
            info!("DAEMON killing");
        }
        None => {}
    }

    Ok(())
}

pub async fn command_to_daemon(command: &DaemonCommand) -> Result<()> {
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:38040").await {
        let (reader, mut writer) = stream.split();
        let mut reader = BufReader::new(reader);
        let mut response = String::new();
        let cmd = serde_json::to_string(command)?;

        writer.write_all(cmd.as_bytes()).await.unwrap();
        reader.read_line(&mut response).await.unwrap();
        println!("{}", response);
    }
    Ok(())
}
