use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerCommand {
    RequestWallpaper,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerMessage {
    pub command: ServerCommand,
    pub content: String,
}

impl ServerMessage {
    pub fn new(command: ServerCommand, content: &str) -> Self {
        ServerMessage {
            command,
            content: content.to_string(),
        }
    }
}
