use anyhow::Result;
use flags::Action;

pub mod commands;
pub mod daemon;
mod flags;
mod network;
mod system;

mod utils;

#[macro_use]
extern crate log;
extern crate simplelog;

use system::config;

#[tokio::main]
async fn main() -> Result<()> {
    let flags = flags::cli_args();
    let crate_name = env!("CARGO_PKG_NAME");

    system::logger::start(&format!("{}.log", crate_name));

    match &flags.command {
        Some(Action::INIT { port }) => {
            let server_port = config::flag_file_default(*port, "port", 37878)?;
            daemon::spawn(server_port).await?;
        }
        Some(Action::STOP) => {
            info!("DAEMON killing");
        }
        None => {}
    }

    Ok(())
}
