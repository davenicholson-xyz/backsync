use anyhow::Result;
use commands::ClientCommand;

pub mod commands;

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
