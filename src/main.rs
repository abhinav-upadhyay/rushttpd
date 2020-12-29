#![allow(dead_code)]

mod server;
mod http;
mod static_handler;

use server::Server;
use static_handler::StaticHandler;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(StaticHandler::new());
}



