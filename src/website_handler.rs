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

    fn generate_landing_page(&mut self) -> Option<String> {
        // TODO: aggrigate repos and display them on page
        // 1. read repos in repo dir
        // 2. generate html page using askama
        // 3. TODO: cache file somewhere
        self.read_file("index.html")
    }

    fn generate_other_pages(&mut self, path: &str) -> Option<String> {
        // TODO: aggrigate repos sub page details
        // 1. get repo name from route
        // 2. generate html page using askama based on sub route requested?
        // 3. TODO: cache file somewhere
        self.read_file(path)
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::OK, self.generate_landing_page()),
                path => match self.generate_other_pages(path) {
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
