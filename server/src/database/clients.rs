use std::net::IpAddr;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Client {
    pub addr: String,
    pub hostname: String,
    pub connected_at: String,
}

pub async fn insert(ip: &str, hostname: &str) -> Result<()> {
    let pool = super::pool();
    sqlx::query(
        r#"
        INSERT INTO clients (addr, hostname, connected_at)
        VALUES (?, ?, datetime('now'))
        ON CONFLICT(addr) 
        DO UPDATE SET connected_at = datetime('now')
    "#,
    )
    .bind(ip)
    .bind(hostname)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn remove(ip: IpAddr) -> Result<()> {
    let pool = super::pool();
    sqlx::query(
        r#"
        UPDATE clients
        SET connected_at = ''
        WHERE addr = ?
    "#,
    )
    .bind(&ip.to_string())
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn all() -> sqlx::Result<Vec<Client>> {
    let pool = super::pool();
    let clients = sqlx::query_as::<_, Client>("SELECT * FROM clients")
        .fetch_all(pool)
        .await?;
    Ok(clients)
}

pub async fn get(addr: &str) -> sqlx::Result<Client> {
    let pool = super::pool();
    let client = sqlx::query_as::<_, Client>("SELECT * FROM clients WHERE addr = ?")
        .bind(addr)
        .fetch_one(pool)
        .await?;
    Ok(client)
}
