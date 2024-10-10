use std::path::PathBuf;

use anyhow::anyhow;
use anyhow::Result;
pub mod wallpaper;

pub async fn create_directory(path: &PathBuf) -> Result<()> {
    let _ = tokio::fs::create_dir_all(&path)
        .await
        .map_err(|e| anyhow!("failed to create wallpaper storage directory: {}", e));
    Ok(())
}
