use anyhow::Result;
use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;

use crate::system::config;

pub trait PathBufExt {
    fn make_string(&self) -> String;
}

impl PathBufExt for PathBuf {
    fn make_string(&self) -> String {
        self.clone().into_os_string().into_string().unwrap()
    }
}

pub fn storage_path(additional_path: &str) -> PathBuf {
    let storage = config::get::<String>("storage").unwrap().unwrap();
    let mut storage_path = PathBuf::new();
    storage_path.push(storage);
    storage_path.push(additional_path);
    return storage_path;
}

pub fn config_path(additional_path: &str) -> PathBuf {
    let crate_name = env!("CARGO_PKG_NAME");
    let homedir = homedir::my_home().unwrap().unwrap().make_string();
    let mut config_path = PathBuf::new();
    config_path.push(homedir);
    config_path.push(format!(".config/{}", crate_name));
    config_path.push(additional_path);
    return config_path;
}

pub fn ext_from_path(path: &str) -> Result<String> {
    Ok(Path::new(path)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap()
        .to_string())
}
