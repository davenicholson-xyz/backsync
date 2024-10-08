mod commands;
mod flags;
mod network;
mod system;

use commands::{commands::ClientCommand, send_to_client};
use rand::seq::IteratorRandom;
use std::{
    fs::{self, File},
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

    let loop_clients = clients.clone();
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(5));

        let mut clients = loop_clients.lock().unwrap();
        if clients.len() > 0 {
            let mut rng = rand::thread_rng();
            let files = fs::read_dir("/Users/dave/projects/backsync/server/wallpaper").unwrap();
            let file = files.choose(&mut rng).unwrap().unwrap();
            let filepath = file.path();
            let filename = filepath.file_name().unwrap();
            let filename_os = filename.to_os_string();
            let file_str = filename_os.to_str().unwrap();
            let command = ClientCommand::SetWallpaper {
                id: String::from(file_str),
            };
            for client in clients.iter_mut() {
                send_to_client(client, &command).unwrap();
            }
        }
    });

    loop {
        tokio::time::sleep(Duration::from_secs(20)).await;
    }
}
