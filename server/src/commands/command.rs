use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Handshake,
    RequestWallpaper {
        id: String,
    },
    SetWallpaper {
        id: String,
    },
    SendWallpaper {
        id: String,
        data: Vec<u8>,
        set: bool,
    },
    ClientInfo {
        ip: String,
        hostname: String,
    },
    Welcome,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::SendWallpaper { id, data, set } => {
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
