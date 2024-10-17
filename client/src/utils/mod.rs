pub mod paths;

use rand::distributions::Alphanumeric;
use rand::Rng;

use crate::system::config;

#[allow(dead_code)]
pub fn seed(len: usize) -> String {
    let s = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect();
    return s;
}

#[allow(dead_code)]
pub fn get_seed() -> String {
    let client_uuid: String;
    if let Some(uuid) = config::get::<String>("uuid").unwrap() {
        client_uuid = uuid;
    } else {
        client_uuid = seed(12);
        config::set("uuid", &client_uuid).unwrap();
    }
    client_uuid
}

pub fn hostname() -> String {
    gethostname::gethostname().into_string().unwrap()
}

pub fn local_ip() -> String {
    local_ip_address::local_ip().unwrap().to_string()
}
