use std::{net::SocketAddr, sync::Arc};

use anyhow::Result;
use tokio::{net::TcpStream, sync::Mutex};

pub async fn add(stream: &Arc<Mutex<TcpStream>>, hostname: &str) -> Result<()> {
    let pool = super::pool();
    let addr = stream.lock().await.peer_addr()?.ip().to_string();
    sqlx::query(
        r#"
        INSERT INTO streams (addr, hostname, connected_at)
        VALUES (?, ?, datetime('now'))
        ON CONFLICT(addr) 
        DO UPDATE SET connected_at = datetime('now')
    "#,
    )
    .bind(&addr)
    .bind(hostname)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn remove(addr: &SocketAddr) -> Result<()> {
    let addr = addr.ip().to_string();
    let pool = super::pool();
    sqlx::query(
        r#"
        UPDATE streams
        SET connected_at = ''
        WHERE addr = ?
    "#,
    )
    .bind(&addr)
    .execute(pool)
    .await?;
    Ok(())
}
