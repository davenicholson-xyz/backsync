use anyhow::Result;
use flags::Commands;

mod daemon;
mod flags;
mod network;
mod system;

#[tokio::main]
async fn main() -> Result<()> {
    let flags = flags::cli_args();
    let cfg_file = system::files::config_file()?;

    match &flags.command {
        Some(Commands::START { port }) => {
            if let Some(flg_port) = *port {
                system::config::set(&cfg_file, "port", flg_port)?;
            }
            let cfg_port: Option<i32> = system::config::get(&cfg_file, "port")?;
            #[cfg(target_family = "unix")]
            daemon::unix::spawn(cfg_port.unwrap_or(37878))?;
        }
        Some(Commands::STOP) => {
            println!("killing the daemon");
        }
        Some(Commands::STATUS) => {
            let daemon_port: Option<i32> = system::config::get(&cfg_file, "daemon_port")?;
            let local_tcp = format!("127.0.0.1:{}", daemon_port.unwrap());
            let _ = network::tcp::query_tcp(&local_tcp, "I AM A QUERY");
        }
        None => {}
    }

    Ok(())
}
