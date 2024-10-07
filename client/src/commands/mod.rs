use anyhow::Result;
use client::ClientCommand;

pub mod client;
pub mod server;

pub fn handle(command: ClientCommand) -> Result<()> {
    match command {
        ClientCommand::Handshake => {
            info!("SERVER: 👋");
        }
        ClientCommand::SetWallpaper { id } => {
            info!("SERVER sent wallaper SET request: {}", id);
        }
    }
    Ok(())
}
