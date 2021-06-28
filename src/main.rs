use crate::config::server_config::Config;
use crate::service::server::Server;

mod config;
mod entities;
mod service;

fn main() {
    let config = Config::new();
    let server = Server::new(config);
    server.serve();
    println!("Hello, world!");
}
