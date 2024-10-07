use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientCommand {
    Handshake,
    SetWallpaper { id: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerCommand {
    Handshake,
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
