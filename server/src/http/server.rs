use std::thread::{self, JoinHandle};

use rust_embed::Embed;
use tiny_http::{Header, Response, Server, StatusCode};

#[derive(Embed)]
#[folder = "static/"]
pub struct StaticFiles;

#[allow(dead_code)]
fn read_static(filename: &str) -> Option<Vec<u8>> {
    StaticFiles::get(filename).map(|file| file.data.to_vec())
}

pub fn start(port: i32) -> JoinHandle<()> {
    thread::spawn(move || {
        http_server(port);
    })
}

pub fn http_server(port: i32) {
    let server = Server::http(format!("127.0.0.1:{}", port)).unwrap();
    loop {
        let request = match server.recv() {
            Ok(req) => req,
            Err(e) => {
                error!("HTTP Server error: {}", e);
                break;
            }
        };

        let path = if request.url() == "/" {
            "index.html"
        } else {
            &request.url()[1..]
        };

        match StaticFiles::get(path) {
            Some(content) => {
                let body = content.data;
                let content_type = get_content_type(path);
                let header =
                    Header::from_bytes(&b"Content-Type"[..], content_type.as_bytes()).unwrap();
                let response = Response::from_data(body).with_header(header);
                request.respond(response).unwrap();
            }
            None => {
                let response =
                    Response::from_string("404 not found").with_status_code(StatusCode(404));
                request.respond(response).unwrap();
            }
        }
    }
}

fn get_content_type(path: &str) -> &'static str {
    if path.ends_with(".html") {
        "text/html"
    } else if path.ends_with(".css") {
        "text/css"
    } else if path.ends_with(".js") {
        "application/javascript"
    } else if path.ends_with(".png") {
        "image/png"
    } else if path.ends_with(".jpg") || path.ends_with(".jpeg") {
        "image/jpeg"
    } else {
        "application/octet-stream"
    }
}
