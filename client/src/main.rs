use anyhow::Result;
use flags::Action;

mod commands;
mod daemon;
mod flags;
mod network;
mod system;

#[tokio::main]
async fn main() -> Result<()> {
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
            println!("killing the daemon");
        }
        None => {}
    }

    Ok(())
}
