use crate::config::server_config::Config;
use crate::service::server::Server;
use std::env;

mod config;
mod entities;
mod service;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let argv: Vec<String> = env::args().collect();
    let config: Config;

    match argv.len() {
        // no arguments passed
        1 => config = Config::new(),
        // one argument passed
        2 => {
            config = Config::new_from_file(argv[1].to_string())?;
        }
        _ => {
            println!("Incorrect params, Try passing one or two arguments!");
            return Err("Incorrect params".into());
        }
    }

    let server = Server::new(config)?;
    server.serve()?;
    Ok(())
}
