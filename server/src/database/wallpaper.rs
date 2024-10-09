use anyhow::Result;

use crate::system::files::UploadData;

pub async fn add(wallpaper: UploadData) -> Result<()> {
    let pool = super::pool();
    sqlx::query(
        r#"
        INSERT INTO wallpapers 
        (id, filename, extension)
        VALUES (?, ?, ?)
    "#,
    )
    .bind(wallpaper.id)
    .bind(wallpaper.filename)
    .bind(wallpaper.extension)
    .execute(pool)
    .await?;
    Ok(())
}
