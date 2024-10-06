use anyhow::Result;
use clap::builder::Str;
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
        Some(Commands::STATUS) => {
            let daemon_port: Option<i32> = system::config::get("daemon_port")?;
            let local_tcp = format!("127.0.0.1:{}", daemon_port.unwrap());
            let _response = network::tcp::query_tcp(&local_tcp, "I AM A QUERY")?;
        }
        Some(Commands::SET { wallpaper }) => {
            let m = Message {
                command: Command::SetWallpaper,
                content: String::from("bars"),
            };
            let response = daemon::commands::send_daemon(m)?;
            println!("{}", response);
        }
        None => {}
    }

    Ok(())
}
