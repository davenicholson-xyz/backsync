use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{network, system};

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    SetWallpaper,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub command: Command,
    pub content: String,
}

pub fn send_daemon(message: Message) -> Result<String> {
    let port: Option<i32> = system::config::get("daemon_port")?;
    let local_tcp = format!("127.0.0.1:{}", port.unwrap());
    let response = network::tcp::send_message(&local_tcp, message)?;
    Ok(response)
}
