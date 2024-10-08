mod commands;
mod flags;
mod network;
mod system;

use std::time::Duration;
use system::config;

use anyhow::Result;

#[macro_use]
extern crate log;
extern crate simplelog;

#[tokio::main]
async fn main() -> Result<()> {
    let crate_name = env!("CARGO_PKG_NAME");

    system::logger::start(&format!("{}.log", crate_name));

    let flags = flags::cli_args();

    config::flag_file_default(flags.storage, "storage", "nice".to_string())?;
    let port = config::flag_file_default(flags.port, "port", 37878)? as i32;

    network::udp::broadcast(port)?;
    network::tcp::start(port)?;

    loop {
        tokio::time::sleep(Duration::from_secs(20)).await;
    }
}
