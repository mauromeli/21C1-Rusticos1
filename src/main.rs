use proyecto_taller_1::run_redis;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut argv: Vec<String> = env::args().collect();
    argv.remove(0);
    run_redis(argv)
}
