pub mod config;
pub mod files;
pub mod logger;
pub mod paths;

use anyhow::Result;
use paths::PathBufExt;

use crate::database;

pub async fn init() -> Result<()> {
    let flags = super::flags::cli_args();
    let crate_name = env!("CARGO_PKG_NAME");
    let homedir = homedir::my_home()?.unwrap().make_string();

    #[cfg(target_family = "unix")]
    let default_path = format!("{}/{}", homedir, crate_name);

    config::flag_file_default(flags.storage, "storage", default_path.clone())?;

    let thumb_dir = paths::storage_path("wallpaper/.thumbs");
    files::create_directory(&thumb_dir).await.unwrap();

    logger::start(&format!("{}/{}.log", &default_path, crate_name));

    database::init().await?;

    Ok(())
}
