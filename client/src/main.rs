use anyhow::Result;
use flags::Commands;
//use tokio::net::UdpSocket;

mod flags;
mod network;

#[tokio::main]
async fn main() -> Result<()> {
    let flags = flags::cli_args();

    match &flags.command {
        Some(Commands::START { port }) => {
            let server_port = port.unwrap_or(37878);
            network::udp::listen_for_broadcast(server_port)?;
        }
        Some(Commands::STOP) => {
            println!("killing the daemon");
        }
        Some(Commands::STATUS) => {
            println!("status of the daemon");
        }
        None => {}
    }

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
