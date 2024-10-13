use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Handshake,
    RequestWallpaper {
        code: String,
    },
    SetWallpaper {
        filename: String,
    },
    SendWallpaper {
        filename: String,
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
            Command::SendWallpaper {
                filename,
                data,
                set,
            } => {
                let _ = data;
                write!(
                    f,
                    "SendWallpaper {{ id: \"{}\", data: [DATA], set: {} }}",
                    filename, set
                )
            }
            _ => {
                write!(f, "{:?}", self)
            }
        }
    }
}
