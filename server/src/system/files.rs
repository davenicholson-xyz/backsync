use std::path::PathBuf;
use std::{fs::File, io::Read, net::TcpStream};

use anyhow::anyhow;
use anyhow::Result;

use crate::commands::{commands::ClientCommand, send_to_client};

use homedir;

use super::config;

pub fn config_file() -> Result<String> {
    let crate_name = env!("CARGO_PKG_NAME");
    if let Some(mut home_dir) = homedir::my_home()? {
        home_dir.push(format!(".config/{}/config.toml", crate_name));
        Ok(home_dir.into_os_string().into_string().unwrap())
    } else {
        Err(anyhow!("Could not find users config directory"))
    }
}

#[allow(dead_code)]
pub fn config_path() -> Result<PathBuf> {
    let crate_name = env!("CARGO_PKG_NAME");
    if let Some(mut home_dir) = homedir::my_home()? {
        home_dir.push(format!(".config/{}", crate_name));
        Ok(home_dir)
    } else {
        Err(anyhow!("Could not find users cache directory"))
    }
}

pub fn send_wallpaper(id: String, stream: &mut TcpStream) -> Result<()> {
    let storage = config::get::<String>("storage")?.unwrap();
    let filepath = format!("{}/{}", storage, id);
    let mut file = File::open(filepath)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let command = ClientCommand::SendWallpaper {
        id,
        data: buffer,
        set: true,
    };

    send_to_client(stream, &command)?;
    Ok(())
}
