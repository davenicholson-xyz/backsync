use anyhow::Result;
use client::ClientCommand;

pub mod client;
pub mod server;

pub fn handle(command: ClientCommand) -> Result<()> {
    match command {
        ClientCommand::Handshake => {
            println!("received handshake from server");
        }
    }
    Ok(())
}
