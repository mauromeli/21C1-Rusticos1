use crate::service::server::Server;

mod entities;
mod service;

fn main() {
    let server = Server::new();
    server.serve();
    println!("Hello, world!");
}
