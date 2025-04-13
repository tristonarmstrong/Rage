use crate::page_generator::{generate_about_page, generate_landing_page};
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

    fn handle_get(&mut self, request: &Request) -> Response {
        match request.path() {
            "/" => self.produce_index(),
            "/about" => self.produce_about(),
            path => match path {
                "bob" => Response::new(StatusCode::OK, Some("DUMY PATH".to_string())),
                _ => Response::new(StatusCode::NotFound, None),
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
