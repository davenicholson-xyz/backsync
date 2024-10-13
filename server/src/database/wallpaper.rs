use anyhow::Result;

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Wallpaper {
    pub id: i32,
    pub code: String,
    pub extension: String,
}

pub async fn add(wallpaper: &Wallpaper) -> Result<()> {
    let pool = super::pool();
    sqlx::query(
        r#"
        INSERT INTO wallpapers 
        (code, extension)
        VALUES (?, ?)
    "#,
    )
    .bind(&wallpaper.code)
    .bind(&wallpaper.extension)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn all() -> sqlx::Result<Vec<Wallpaper>> {
    let pool = super::pool();
    let streams = sqlx::query_as::<_, Wallpaper>("SELECT * FROM wallpapers")
        .fetch_all(pool)
        .await?;
    Ok(streams)
}

pub async fn get(code: &str) -> sqlx::Result<Wallpaper> {
    let pool = super::pool();
    let wallpaper = sqlx::query_as::<_, Wallpaper>("SELECT * FROM wallpapers WHERE code = ?")
        .bind(code)
        .fetch_one(pool)
        .await?;
    Ok(wallpaper)
}

pub async fn delete(code: &str) -> Result<()> {
    let pool = super::pool();
    sqlx::query("DELETE FROM wallpapers WHERE code = ?")
        .bind(code)
        .execute(pool)
        .await?;
    Ok(())
}
