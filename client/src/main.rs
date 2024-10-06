use std::ops::Deref;

use anyhow::Result;
use daemon::commands::{Command, Message};
use flags::Commands;

mod daemon;
mod flags;
mod network;
mod system;

#[tokio::main]
async fn main() -> Result<()> {
    let flags = flags::cli_args();

    match &flags.command {
        Some(Commands::START { port }) => {
            if let Some(flg_port) = *port {
                system::config::set("port", flg_port)?;
            }
            let cfg_port: Option<i32> = system::config::get("port")?;
            #[cfg(target_family = "unix")]
            daemon::unix::spawn(cfg_port.unwrap_or(37878))?;
        }
        Some(Commands::STOP) => {
            println!("killing the daemon");
        }
        Some(Commands::SET { wallpaper }) => {
            if let Some(wp) = wallpaper {
                let m = Message {
                    command: Command::SetWallpaper,
                    content: wp.deref().to_string(),
                };
                let response = daemon::commands::send_daemon(m)?;
                println!("{}", response);
            }
        }
        None => {}
    }

    Ok(())
}
