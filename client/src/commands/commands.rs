use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientCommand {
    Handshake,
    SetWallpaper {
        id: String,
    },
    SendWallpaper {
        id: String,
        data: Vec<u8>,
        set: bool,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerCommand {
    Handshake,
    RequestWallpaper { id: String },
}

impl fmt::Display for ClientCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for ServerCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
