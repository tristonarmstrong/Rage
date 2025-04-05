use server::Server;
use std::env;
use website_handler::WebsiteHandler;

mod http;
mod server;
mod website_handler;
mod generation_templates;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC").unwrap_or(default_path);

    let address = String::from("localhost:8080");
    let server = Server::new(address);

    server.run(WebsiteHandler::new(public_path));
}
