use anyhow::anyhow;
use anyhow::Result;

pub fn split_filename(filename: &str) -> Result<(&str, &str)> {
    println!("splitting filename: {}", filename);
    if let Some(pos) = filename.rfind(".") {
        let (name, ext) = filename.split_at(pos);
        Ok((name, &ext[1..]))
    } else {
        Err(anyhow!("Could not split the filename: {}", filename))
    }
}
