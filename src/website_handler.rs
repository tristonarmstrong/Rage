use std::fs;

use crate::{
    http::{Method, Request, Response, StatusCode},
    server::Handler,
};

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    return fs::read_to_string(path).ok();
                }

                println!("Directory traversal attack attempted: {}", file_path);
                None
            }

            Err(_) => None,
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::OK, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::OK, self.read_file("hello.html")),
                path => match self.read_file(path) {
                    Some(file) => Response::new(StatusCode::OK, Some(file)),
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
            Method::POST => todo!(),
            Method::OPTIONS => todo!(),
            Method::PUT => todo!(),
            Method::DELETE => todo!(),
            Method::HEAD => todo!(),
            Method::CONNECT => todo!(),
            Method::TRACE => todo!(),
            Method::PATCH => todo!(),
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
