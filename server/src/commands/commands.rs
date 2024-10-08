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
    Handshake { hostname: String },
    RequestWallpaper { id: String },
}

impl fmt::Display for ClientCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientCommand::SendWallpaper { id, data, set } => {
                let _ = data;
                write!(
                    f,
                    "SendWallpaper {{ id: \"{}\", data: [DATA], set: {} }}",
                    id, set
                )
            }
            _ => {
                write!(f, "{:?}", self)
            }
        }
    }
}

impl fmt::Display for ServerCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
