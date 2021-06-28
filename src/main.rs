mod entities;
mod service;
mod config;

use crate::config::server_config::Config;

fn main() {
    let path = String::from("src/file.conf");
    let cfg = Config::new_from_file(path);
    println!("{:?}", cfg);
}
