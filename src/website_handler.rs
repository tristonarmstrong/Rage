use std::fs;
use std::path::Path;

use crate::page_generator::{generate_about_page, generate_landing_page};
use crate::utils::Logger;
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

    fn produce_index(&self) -> Response {
        Response::new(StatusCode::OK, Some(generate_landing_page().unwrap()))
    }

    fn produce_about(&self) -> Response {
        Response::new(StatusCode::OK, Some(generate_about_page().unwrap()))
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    return fs::read_to_string(path).ok();
                }
                Logger::warn(
                    format!("Directory traversal attack attempted: {}", file_path).as_str(),
                );
                None
            }
            Err(_) => None,
        }
    }

    fn handle_get(&mut self, request: &Request) -> Response {
        match request.path() {
            "/" => self.produce_index(),
            "/about" => self.produce_about(),
            path => match self.read_file(path) {
                Some(file) => Response::new(StatusCode::OK, Some(file)),
                _ => {
                    Logger::warn(
                        format!("Attempt to retrieve file {} failed", request.path()).as_str(),
                    );
                    Response::new(StatusCode::NotFound, None)
                }
            },
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => self.handle_get(request),
            Method::POST => todo!(),
            Method::OPTIONS => todo!(),
            Method::PUT => todo!(),
            Method::DELETE => todo!(),
            Method::HEAD => todo!(),
            Method::CONNECT => todo!(),
            Method::TRACE => todo!(),
            Method::PATCH => todo!(),
        }
    }
}
