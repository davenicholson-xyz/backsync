use anyhow::Result;

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Wallpaper {
    pub id: String,
    pub filename: String,
    pub extension: String,
}

pub async fn add(wallpaper: &Wallpaper) -> Result<()> {
    let pool = super::pool();
    sqlx::query(
        r#"
        INSERT INTO wallpapers 
        (id, filename, extension)
        VALUES (?, ?, ?)
    "#,
    )
    .bind(&wallpaper.id)
    .bind(&wallpaper.filename)
    .bind(&wallpaper.extension)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn all() -> sqlx::Result<Vec<Wallpaper>> {
    let pool = super::pool();
    let streams = sqlx::query_as::<_, Wallpaper>("SELECT id, filename, extension FROM wallpapers")
        .fetch_all(pool)
        .await?;
    Ok(streams)
}

pub async fn get(id: &str) -> sqlx::Result<Wallpaper> {
    let pool = super::pool();
    let wallpaper = sqlx::query_as::<_, Wallpaper>(
        "SELECT id, filename, extension FROM wallpapers WHERE id = ?",
    )
    .bind(id)
    .fetch_one(pool)
    .await?;
    Ok(wallpaper)
}

pub async fn delete(id: &str) -> Result<()> {
    let pool = super::pool();
    sqlx::query("DELETE FROM wallpapers WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
