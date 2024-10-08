mod commands;
mod database;
mod flags;
mod http;
mod network;
mod system;

use system::config;

use anyhow::Result;

#[macro_use]
extern crate log;
extern crate simplelog;

#[tokio::main]
async fn main() -> Result<()> {
    let flags = flags::cli_args();
    let crate_name = env!("CARGO_PKG_NAME");

    system::logger::start(&format!("{}.log", crate_name));

    config::flag_file_default(flags.storage, "storage", "nice".to_string())?;
    let network_port = config::flag_file_default(flags.port, "port", 37878)? as i32;

    network::udp::broadcast(network_port)?;
    network::tcp::start(network_port)?;

    let http_port = config::flag_file_default(flags.web_port, "web_port", 3001)? as i32;
    http::server::start(http_port).await;

    Ok(())
}
