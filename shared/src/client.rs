use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientCommand {
    Handshake,
    SetWallpaper,
    Test,
    ImageData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientMessage {
    pub command: ClientCommand,
    pub content: String,
}

impl ClientMessage {
    pub fn new(command: ClientCommand, content: &str) -> Self {
        ClientMessage {
            command,
            content: content.to_string(),
        }
    }
}
