use crate::config::server_config::Config;
use crate::service::server::Server;

mod config;
mod entities;
mod protocol;
mod service;

pub fn run_redis(argv: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let config: Config;

    match argv.len() {
        // no arguments passed
        0 => config = Config::new(),
        // one argument passed
        1 => {
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
