use anyhow::Result;

use crate::{
    //commands::{command::Command, send_to_server},
    system,
};

use super::files;

pub fn set_wallpaper(img: &str) -> Result<()> {
    debug!("SETTING WP to {}", img);
    let mut cachepath = files::cache_path()?;
    cachepath.push(img);
    if !cachepath.exists() {
        debug!("WP {} does not exist in cache. Requesting...", img);
        //send_to_server(Command::RequestWallpaper {
        //    id: String::from(img),
        //})?;
    } else {
        info!("SETTING: {}", img);
        if let Some(script) = system::config::get::<String>("post_script")? {
            let parsed_command = shlex::split(&script).expect("Failed to parse external script");
            if let Some((command, args)) = parsed_command.split_first() {
                let _status = std::process::Command::new(command)
                    .args(args)
                    .arg(cachepath.display().to_string())
                    .status()
                    .expect("Failed to execute external script");
            };
        } else {
            setwallpaper::from_file(&cachepath.into_os_string().into_string().unwrap())?;
        }
    }
    Ok(())
}
