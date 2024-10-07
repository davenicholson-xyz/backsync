use std::fs::File;

use anyhow::Result;
use flags::Action;

mod commands;
mod daemon;
mod flags;
mod network;
mod system;

use simplelog::CombinedLogger;

#[macro_use]
extern crate log;
extern crate simplelog;

use simplelog::*;

#[tokio::main]
async fn main() -> Result<()> {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("bs-client.log").unwrap(),
        ),
    ])
    .unwrap();

    let flags = flags::cli_args();

    match &flags.command {
        Some(Action::INIT { port }) => {
            if let Some(flg_port) = *port {
                system::config::set("port", flg_port)?;
            }
            let cfg_port: Option<i32> = system::config::get("port")?;
            daemon::spawn(cfg_port.unwrap_or(37878))?;
        }
        Some(Action::STOP) => {
            info!("DAEMON killing");
        }
        None => {}
    }

    Ok(())
}
