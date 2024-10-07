mod commands;
mod flags;
mod network;

use std::{
    fs::File,
    net::{SocketAddr, TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use anyhow::Result;
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
            File::create("bs-server.log").unwrap(),
        ),
    ])
    .unwrap();

    let flags = flags::cli_args();

    let local_ip = local_ip_address::local_ip()?;
    let port = flags.port.unwrap_or(37878);
    let broadcast_address: SocketAddr = format!("255.255.255.255:{}", port).parse()?;
    let msg = format!("SVR::{}:{}", local_ip, port);

    network::udp::broadcast(broadcast_address, msg)?;

    let listener = TcpListener::bind(format! {"{}:{}", local_ip, port})?;
    let clients: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));

    let tcp_clients = clients.clone();
    thread::spawn(move || {
        let _ = network::tcp::handle_client(listener, tcp_clients);
    });

    loop {
        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}
