use server::Server;
use std::env;
use website_handler::WebsiteHandler;

mod generation_templates;
mod http;
mod page_generator;
mod server;
mod utils;
mod website_handler;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC").unwrap_or(default_path);

    let address = String::from("localhost:8080");
    let server = Server::new(address);

    server.run(WebsiteHandler::new(public_path));
}
