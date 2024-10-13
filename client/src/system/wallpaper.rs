use anyhow::Result;

use crate::{commands::command::Command, daemon::send_to_server, system};

use super::files;

pub async fn set_wallpaper(img: &str) -> Result<()> {
    info!("SETTING WP to {}", img);
    let mut cachepath = files::cache_path()?;
    cachepath.push(format!("{}", img));
    if !cachepath.exists() {
        info!("WP {} does not exist in cache. Requesting...", img);
        let filepath = cachepath.as_path();
        let filename = filepath.file_stem().unwrap();
        println!("filename: {}", filename.to_str().unwrap());
        send_to_server(Command::RequestWallpaper {
            code: String::from(filename.to_str().unwrap()),
        })
        .await
        .unwrap();
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
