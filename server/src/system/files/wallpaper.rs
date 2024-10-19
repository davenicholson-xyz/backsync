use std::fs::File;
use std::io::Cursor;
use std::io::Read;
use std::path::Path;

use anyhow::anyhow;
use image::ImageFormat;

use image::ImageReader;

use axum::extract::Multipart;

use crate::commands::command::Command;
use crate::commands::send_to_client;
use crate::database;
use crate::database::wallpaper::Wallpaper;
use crate::system::config;
use crate::system::paths;
use crate::system::paths::storage_path;
use crate::utils;

use anyhow::Result;

pub async fn delete_wallpaper(id: &str, ext: &str) -> Result<()> {
    let wallpaper_path = paths::storage_path(&format!("wallpaper/{}.{}", id, ext));
    let thumbnail_path = paths::storage_path(&format!("wallpaper/.thumbs/{}.jpg", id));
    std::fs::remove_file(wallpaper_path)?;
    std::fs::remove_file(thumbnail_path)?;
    Ok(())
}

pub async fn send_wallpaper(code: &str, uuid: &str) -> Result<()> {
    info!("SENDING wallpaper.... {}", code);
    let wallpaper = database::wallpaper::get(&code).await?;
    let storage = config::get::<String>("storage")?.unwrap();
    let filepath = format!(
        "{}/wallpaper/{}.{}",
        storage, wallpaper.code, wallpaper.extension
    );
    let mut file = File::open(&filepath)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let filepath_p = Path::new(&filepath);
    let filename_p = filepath_p.file_name().unwrap();

    let command = Command::SendWallpaper {
        filename: String::from(filename_p.to_str().unwrap()),
        data: buffer,
        set: true,
    };

    let client = database::clients::get_by_uuid(&uuid).await?;
    send_to_client(&client.uuid, &command).await?;

    Ok(())
}

pub async fn save_image_from_url(url: &str) -> Result<Wallpaper> {
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;
    let filename = utils::filename_from_url(&url).unwrap();
    let wp = save_image_bytes(&filename, &bytes).await?;
    Ok(wp)
}

pub async fn save_image_bytes(filename: &str, image_data: &[u8]) -> Result<Wallpaper> {
    let upload_dir = storage_path("wallpaper");
    let thumb_dir = storage_path("wallpaper/.thumbs");
    let file_id = utils::seed(8);

    let file_ext = paths::ext_from_path(&filename)?;

    let file_path = upload_dir.join(format!("{}.{}", file_id, file_ext));
    let thumb_path = thumb_dir.join(format!("{}.jpg", file_id));

    let mut file_contents = Vec::new();
    file_contents.extend_from_slice(&image_data);
    let img = ImageReader::new(Cursor::new(&file_contents))
        .with_guessed_format()?
        .decode()?;

    let thumbnail = img.thumbnail(300, 300);

    img.save(&file_path)?;
    thumbnail
        .to_rgb8()
        .save_with_format(&thumb_path, ImageFormat::Jpeg)?;

    Ok(Wallpaper {
        id: 0,
        code: file_id,
        extension: file_ext,
    })
}

pub async fn upload_image(mut multipart: Multipart) -> Result<Wallpaper> {
    let upload_dir = storage_path("wallpaper");
    let thumb_dir = storage_path("wallpaper/.thumbs");
    let file_id = utils::seed(8);

    while let Some(field) = multipart.next_field().await.unwrap() {
        let filename = field.file_name().unwrap().to_string();
        let data = field.bytes().await?;

        let file_ext = paths::ext_from_path(&filename)?;

        let file_path = upload_dir.join(format!("{}.{}", file_id, file_ext));
        let thumb_path = thumb_dir.join(format!("{}.jpg", file_id));

        let mut file_contents = Vec::new();
        file_contents.extend_from_slice(&data);
        let img = ImageReader::new(Cursor::new(&file_contents))
            .with_guessed_format()?
            .decode()?;

        let thumbnail = img.thumbnail(300, 300);

        img.save(&file_path)?;
        thumbnail
            .to_rgb8()
            .save_with_format(&thumb_path, ImageFormat::Jpeg)?;

        return Ok(Wallpaper {
            id: 0,
            code: file_id,
            extension: file_ext,
        });
    }

    Err(anyhow!("no file"))
}

//pub async fn set_wallpaper_all(id: &str) -> Result<()> {
//    info!("set wallpaper all fn");
//    let clients = CLIENTS.lock().await;
//    for client in clients.iter() {
//        let c = Arc::clone(client);
//        //send_to_client(c, &Command::SetWallpaper { id: id.to_string() }).await?;
//        send_to_client(c, &Command::Welcome).await?;
//    }
//    Ok(())
//}
