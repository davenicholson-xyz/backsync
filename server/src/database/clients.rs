use std::net::IpAddr;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::http;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Client {
    pub id: i32,
    pub uuid: String,
    pub addr: String,
    pub hostname: String,
    pub connected_at: String,
    pub locked: bool,
    pub syncwall: Option<String>,
    pub wallpaper_code: Option<String>,
}

pub async fn insert(uuid: &str, ip: &str, hostname: &str) -> Result<()> {
    let pool = super::pool();

    sqlx::query(
        r#"
        INSERT INTO 
            clients (uuid, addr, hostname, connected_at)
        VALUES 
            (?, ?, ?, datetime('now'))
        ON CONFLICT(uuid) 
        DO UPDATE SET 
            addr = excluded.addr,
            connected_at = datetime('now')
    "#,
    )
    .bind(uuid)
    .bind(ip)
    .bind(hostname)
    .execute(pool)
    .await?;

    Ok(())
}

#[allow(dead_code)]
pub async fn update_field(uuid: &str, field: &str, value: &str) -> Result<()> {
    let pool = super::pool();
    let query = format!("UPDATE clients SET {} = $1 WHERE uuid = $2", field);
    sqlx::query(&query)
        .bind(value)
        .bind(uuid)
        .execute(pool)
        .await?;
    http::websocket::client_update().await?;

    Ok(())
}

pub async fn delete(uuid: &str) -> Result<()> {
    let pool = super::pool();
    sqlx::query("DELETE FROM clients WHERE uuid = ?")
        .bind(uuid)
        .execute(pool)
        .await?;
    http::websocket::client_update().await?;
    Ok(())
}

pub async fn set_wallpaper(uuid: &str, code: &str) -> Result<()> {
    let pool = super::pool();
    sqlx::query(
        r#"
        UPDATE 
            clients
        SET 
            wallpaper = (SELECT id FROM wallpapers WHERE code = ?),
            syncwall = NULL
        WHERE 
            uuid = ?
    "#,
    )
    .bind(&code.to_string())
    .bind(&uuid)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn disconnected_by_ip(ip: IpAddr) -> Result<()> {
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
    http::websocket::client_update().await?;
    Ok(())
}

pub async fn all() -> sqlx::Result<Vec<Client>> {
    let pool = super::pool();
    let clients = sqlx::query_as::<_, Client>(
        r#"
        SELECT 
            clients.id, 
            clients.uuid, 
            clients.addr, clients.hostname, 
            clients.connected_at, 
            clients.locked,
            clients.syncwall,
            wallpapers.code AS wallpaper_code 
        FROM 
            clients 
        LEFT JOIN 
            wallpapers ON clients.wallpaper = wallpapers.id
    "#,
    )
    .fetch_all(pool)
    .await?;
    Ok(clients)
}

pub async fn all_online() -> sqlx::Result<Vec<Client>> {
    let pool = super::pool();
    let clients = sqlx::query_as::<_, Client>(
        r#"
        SELECT 
            clients.id, 
            clients.uuid, 
            clients.addr, 
            clients.hostname, 
            clients.connected_at, 
            clients.locked, 
            clients.syncwall,
            wallpapers.code AS wallpaper_code 
        FROM 
            clients 
        LEFT JOIN 
            wallpapers ON clients.wallpaper = wallpapers.id
        WHERE connected_at != '';
    "#,
    )
    .fetch_all(pool)
    .await?;
    Ok(clients)
}

pub async fn get_by_addr(addr: &str) -> sqlx::Result<Client> {
    let pool = super::pool();
    let client = sqlx::query_as::<_, Client>(
        r#"
        SELECT 
            clients.id, 
            clients.uuid, 
            clients.addr, 
            clients.hostname, 
            clients.connected_at, 
            clients.locked,
            clients.syncwall,
            wallpapers.code AS wallpaper_code 
        FROM 
            clients 
        LEFT JOIN 
            wallpapers ON clients.wallpaper = wallpapers.id
        WHERE addr = ?;
    "#,
    )
    .bind(addr)
    .fetch_one(pool)
    .await?;
    Ok(client)
}

#[allow(dead_code)]
pub async fn get_by_uuid(uuid: &str) -> sqlx::Result<Client> {
    let pool = super::pool();
    let client = sqlx::query_as::<_, Client>(
        r#"
        SELECT 
            clients.id, 
            clients.uuid, 
            clients.addr, 
            clients.hostname, 
            clients.connected_at, 
            clients.locked,
            clients.syncwall,
            wallpapers.code AS wallpaper_code 
        FROM 
            clients 
        LEFT JOIN 
            wallpapers ON clients.wallpaper = wallpapers.id
        WHERE uuid = ?;
    "#,
    )
    .bind(uuid)
    .fetch_one(pool)
    .await?;
    Ok(client)
}

//pub async fn get_by_id(id: i32) -> sqlx::Result<Client> {
//    let pool = super::pool();
//    let client = sqlx::query_as::<_, Client>(
//    r#"
//        SELECT clients.id, clients.uuid, clients.addr, clients.hostname, clients.connected_at, wallpapers.code AS wallpaper_code
//        FROM clients
//        LEFT JOIN wallpapers ON clients.wallpaper = wallpapers.id
//        WHERE clients.id = ?;
//    "#
//        )
//        .bind(id)
//        .fetch_one(pool)
//        .await?;
//    Ok(client)
//}
//
pub async fn startup_clean() -> Result<()> {
    let pool = super::pool();
    sqlx::query(
        r#"
        UPDATE clients SET connected_at = NULL
    "#,
    )
    .execute(pool)
    .await?;
    Ok(())
}
