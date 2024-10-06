use anyhow::Result;

pub fn listen_for_broadcast(port: i32) -> Result<()> {
    println!("listening for udp broadcast on port {}", port);
    Ok(())
}
