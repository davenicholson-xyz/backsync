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

    if let Some(flag_storage) = flags.storage {
        config::set("storage", flag_storage)?;
    }
    config::set_if_none("storage", format!("/var/lib/{}", crate_name))?;

    if let Some(flg_port) = flags.port {
        system::config::set("port", flg_port)?;
    }
    let cfg_port: Option<i32> = system::config::get("port")?;
    let port = cfg_port.unwrap_or(37878);

    network::udp::broadcast(port)?;
    network::tcp::start(port)?;

    loop {
        tokio::time::sleep(Duration::from_secs(20)).await;
    }
}
