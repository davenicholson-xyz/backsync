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

            daemon::unix::spawn(cfg_port.unwrap_or(37878))?;
        }
        Some(Commands::STOP) => {
            println!("killing the daemon");
        }
        Some(Commands::STATUS) => {
            println!("status of the daemon");
        }
        None => {}
    }

    //network::udp::listen_for_broadcast(cfg_port.unwrap_or(37878))?;

    //let port = flags.port.unwrap_or(37878);
    //
    //let udp_string = format!("0.0.0.0:{}", port);
    //let socket = UdpSocket::bind(udp_string).await?;
    //let mut buf = [0; 1024];
    //
    //println!("listening for udp on port {}", port);
    //
    //let (len, _addr) = socket.recv_from(&mut buf).await?;
    //let msg = String::from_utf8_lossy(&buf[..len]);
    //println!("Received: {}", msg);
    Ok(())
}
