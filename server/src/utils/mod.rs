use rand::distributions::Alphanumeric;
use rand::Rng;

pub fn seed(len: usize) -> String {
    let s = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect();
    return s;
}

pub fn split_filename(filename: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = filename.rsplitn(2, '.').collect();
    if parts.len() == 2 {
        let ext = parts[0].to_string();
        let id = parts[1].to_string();
        return Some((id, ext));
    }
    None
}
