use std::{net::SocketAddr, sync::Arc};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tokio::{net::TcpStream, sync::Mutex};

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Stream {
    pub addr: String,
    pub hostname: String,
    pub connected_at: String,
}

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

pub async fn all() -> sqlx::Result<Vec<Stream>> {
    let pool = super::pool();
    let streams = sqlx::query_as::<_, Stream>("SELECT addr, hostname, connected_at FROM streams")
        .fetch_all(pool)
        .await?;
    Ok(streams)
}
