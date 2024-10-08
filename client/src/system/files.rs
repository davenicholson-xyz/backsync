use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use anyhow::anyhow;
use anyhow::Result;
use homedir;

pub fn config_file() -> Result<String> {
    let crate_name = env!("CARGO_PKG_NAME");
    if let Some(mut home_dir) = homedir::my_home()? {
        home_dir.push(format!(".config/{}/config.toml", crate_name));
        Ok(home_dir.into_os_string().into_string().unwrap())
    } else {
        Err(anyhow!("Could not find users config directory"))
    }
}

#[allow(dead_code)]
pub fn config_path() -> Result<PathBuf> {
    let crate_name = env!("CARGO_PKG_NAME");
    if let Some(mut home_dir) = homedir::my_home()? {
        home_dir.push(format!(".config/{}", crate_name));
        Ok(home_dir)
    } else {
        Err(anyhow!("Could not find users cache directory"))
    }
}
//pub fn config_file() -> Result<String> {
//    let crate_name = env!("CARGO_PKG_NAME");
//    if let Some(mut home_dir) = homedir::my_home()? {
//        home_dir.push(format!(".config/{}/config.toml", crate_name));
//        Ok(home_dir.into_os_string().into_string().unwrap())
//    } else {
//        Err(anyhow!("Could not find users config directory"))
//    }
//}
//
#[allow(dead_code)]
pub fn cache_path() -> Result<PathBuf> {
    let crate_name = env!("CARGO_PKG_NAME");
    if let Some(mut home_dir) = homedir::my_home()? {
        home_dir.push(format!(".cache/{}", crate_name));
        Ok(home_dir)
    } else {
        Err(anyhow!("Could not find users cache directory"))
    }
}

pub fn save_wallpaper(id: String, data: Vec<u8>) -> Result<()> {
    let mut cachepath = cache_path()?;
    cachepath.push(&id);
    let mut file = File::create(cachepath)?;
    file.write_all(&data)?;
    Ok(())
}
