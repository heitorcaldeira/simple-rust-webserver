use crate::http::{HttpMethod, Request, Response, StatusCode};
use super::server::Handler;
use std::fs;

pub struct WebSiteHandler {
    public_path: String,
}

impl WebSiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Trying to get files behind the public path");
                    None
                }
            },
            Err(_) => None
        }
    }
}

impl Handler for WebSiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            HttpMethod::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                file => match self.read_file(file) {
                    Some(expr) => Response::new(StatusCode::Ok, Some(expr)),
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
