pub mod config;
pub mod files;
pub mod logger;
pub mod paths;

use std::path::PathBuf;

use anyhow::Result;
use paths::PathBufExt;

use crate::database;

pub async fn init() -> Result<()> {
    let flags = super::flags::cli_args();
    let crate_name = env!("CARGO_PKG_NAME");
    let homedir = homedir::my_home()?.unwrap().make_string();

    #[cfg(target_family = "unix")]
    let default_path = format!("{}/.local/share/{}", homedir, crate_name);
    let storage_path = config::flag_file_default(flags.storage, "storage", default_path.clone())?;
    files::create_directory(&PathBuf::from(storage_path))
        .await
        .unwrap();

    //TODO: Choose wallpaper path in config
    let thumb_dir = paths::storage_path("wallpaper/.thumbs");
    files::create_directory(&thumb_dir).await.unwrap();

    let log_dir = paths::storage_path("logs");
    files::create_directory(&log_dir).await.unwrap();
    logger::start(&format!("{}/{}.log", &log_dir.make_string(), crate_name));

    database::init().await?;

    Ok(())
}
