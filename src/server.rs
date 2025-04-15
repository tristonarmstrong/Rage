use crate::http::{ParserError, Request, Response, StatusCode};
use crate::utils::Logger;
use std::convert::TryFrom;
use std::io::Read;
use std::net::{TcpListener, TcpStream};

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParserError) -> Response {
        Logger::err(format!("Failed to handle request: {e:?}").as_str());
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Server listening on: {}", &self.addr[10..]);
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => Self::handle_client(&mut stream, &mut handler),
                Err(e) => Logger::err(format!("Failed to establish a connection: {}", e).as_str()),
            }
        }
    }

    pub fn handle_client(stream: &mut TcpStream, handler: &mut impl Handler) {
        let mut buffer = [0; 1024];

        match stream.read(&mut buffer) {
            Ok(_) => {
                let response = match Request::try_from(&buffer[..]) {
                    Ok(request) => handler.handle_request(&request),
                    Err(e) => handler.handle_bad_request(&e),
                };

                if let Err(e) = response.send(stream) {
                    Logger::err(format!("Failed to send response: {}", e).as_str());
                }
            }
            Err(e) => Logger::err(format!("Failed to read from connection: {}", e).as_str()),
        }
    }
}
