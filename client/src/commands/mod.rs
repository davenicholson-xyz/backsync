use anyhow::Result;
use command::Command;

use crate::{
    daemon::send_to_server,
    system::{
        files,
        wallpaper::{self, set_wallpaper},
    },
};

pub mod command;

pub async fn handle(command: Command) -> Result<()> {
    info!("Received Command::{}", &command);
    match command {
        Command::Handshake => {
            let ip = local_ip_address::local_ip()?.to_string();
            let hostname = gethostname::gethostname().into_string().unwrap();
            send_to_server(Command::ClientInfo { ip, hostname }).await?;
        }
        Command::SetWallpaper { filename } => {
            info!("SERVER sent wallaper SET request: {}", filename);
            set_wallpaper(&filename).await?;
        }
        Command::SendWallpaper {
            filename,
            data,
            set,
        } => {
            files::save_wallpaper(filename.clone(), data)?;
            if set {
                wallpaper::set_wallpaper(&filename).await?;
            }
        }
        _ => {}
    }
    Ok(())
}
