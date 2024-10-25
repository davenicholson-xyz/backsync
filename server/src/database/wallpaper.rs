use anyhow::Result;

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Wallpaper {
    pub id: i32,
    pub code: String,
    pub extension: String,
    pub origin: String,
}

pub async fn add(wallpaper: &Wallpaper) -> Result<()> {
    let pool = super::pool();
    sqlx::query(
        r#"
        INSERT INTO wallpapers 
        (code, extension, origin)
        VALUES (?, ?, ?)
    "#,
    )
    .bind(&wallpaper.code)
    .bind(&wallpaper.extension)
    .bind(&wallpaper.origin)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn all() -> sqlx::Result<Vec<Wallpaper>> {
    let pool = super::pool();
    let wallpapers = sqlx::query_as::<_, Wallpaper>("SELECT * FROM wallpapers")
        .fetch_all(pool)
        .await?;
    Ok(wallpapers)
}

pub async fn count() -> sqlx::Result<i64> {
    let pool = super::pool();
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM wallpapers")
        .fetch_one(pool)
        .await?;
    Ok(count.0)
}

pub async fn page(p: u32) -> sqlx::Result<Vec<Wallpaper>> {
    let pool = super::pool();
    let per_page = 24;
    let offset = (p.saturating_sub(1) * per_page) as i64;
    let wallpapers =
        sqlx::query_as::<_, Wallpaper>("SELECT * FROM wallpapers ORDER BY id LIMIT $1 OFFSET $2")
            .bind(per_page as i64)
            .bind(offset)
            .fetch_all(pool)
            .await?;
    Ok(wallpapers)
}

pub async fn get_by_origin(origin: &str) -> sqlx::Result<Wallpaper> {
    let pool = super::pool();
    let wallpaper = sqlx::query_as::<_, Wallpaper>("SELECT * FROM wallpapers WHERE origin = ?")
        .bind(origin)
        .fetch_one(pool)
        .await?;
    Ok(wallpaper)
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
