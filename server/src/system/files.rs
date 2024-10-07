use std::{fs::File, io::Read, net::TcpStream};

use anyhow::Result;

use crate::commands::{commands::ClientCommand, send_to_client};

pub fn send_wallpaper(id: String, stream: &mut TcpStream) -> Result<()> {
    let filepath = format!("/Users/dave/projects/backsync/server/wallpaper/{}", id);
    debug!("{}", filepath);
    let mut file = File::open(filepath)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let command = ClientCommand::SendWallpaper {
        id,
        data: buffer,
        set: true,
    };

    info!("about to send the wallpaper to the client");
    send_to_client(stream, &command)?;
    Ok(())
}
