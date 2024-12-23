use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub enum DaemonCommand {
    Lock,
    Unlock,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Handshake,
    RequestWallpaper {
        uuid: String,
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
    ConfirmWallpaper {
        uuid: String,
        code: String,
    },
    ClientInfo {
        uuid: String,
        ip: String,
        hostname: String,
    },
    ClientConfirm {
        uuid: String,
    },
    ClientLock {
        uuid: String,
    },
    ClientUnlock {
        uuid: String,
    },
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
