use anyhow::Result;

use crate::{
    commands::command::Command,
    daemon::send_to_server,
    system::{self, config},
    utils,
};

use super::files;

pub async fn set_wallpaper(img: &str) -> Result<()> {
    info!("SETTING WP to {}", img);
    let uuid = config::get::<String>("uuid").unwrap().unwrap();
    let mut cachepath = files::cache_path()?;
    cachepath.push(format!("{}", img));
    if !cachepath.exists() {
        info!("WP {} does not exist in cache. Requesting...", img);
        let (code, _ext) =
            utils::paths::split_filename(img).expect("could not get path, ext from filename");
        send_to_server(Command::RequestWallpaper {
            uuid,
            code: String::from(code),
        })
        .await?;
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
        let (code, _ext) =
            utils::paths::split_filename(img).expect("could not get path, ext from filename");
        send_to_server(Command::ConfirmWallpaper {
            uuid,
            code: code.to_string(),
        })
        .await?;
    }
    Ok(())
}
