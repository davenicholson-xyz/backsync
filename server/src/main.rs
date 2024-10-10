mod commands;
mod database;
mod flags;
mod http;
mod network;
mod system;
mod utils;

use system::config;

use anyhow::Result;

#[macro_use]
extern crate log;
extern crate simplelog;

#[tokio::main]
async fn main() -> Result<()> {
    system::init().await?;

    let flags = flags::cli_args();
    let network_port = config::flag_file_default(flags.port, "port", 37878)? as i32;
    let http_port = config::flag_file_default(flags.web_port, "web_port", 3001)? as i32;

    tokio::try_join!(
        async {
            network::udp::broadcast(network_port)?;
            Ok::<_, anyhow::Error>(())
        },
        async {
            network::tcp::start(network_port).await?;
            Ok::<_, anyhow::Error>(())
        },
        async {
            http::server::start(http_port).await;
            Ok::<_, anyhow::Error>(())
        }
    )?;
    Ok(())
}
