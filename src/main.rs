use std::collections::HashMap;

#[derive(Debug)]
struct Redis {
    db: HashMap<String, String>,
}

#[derive(Debug)]
pub enum Command {
    Key(u32)
}

impl Redis {
    pub fn new() -> Self {
        let mut map: HashMap<String, String> = HashMap::new();

        return Self {
            db: map
        }
    }

    pub fn execute(mut self, command: &Command, key: &String, value: &String) -> Result<String, String> {

        match command {
            Command::Key(1) => {
                return Ok("pong".to_string());
            }
            Command::Key(2) => {
                match self.db.get(key) {
                    Some(return_value) => Ok(return_value.to_string()),
                    None => Err("Not Found".to_string())
                }
            }
            Command::Key(3) => {
                match self.db.insert(key.to_string(), value.to_string()) {
                    Some(_) => Ok("Se guardo".to_string()),
                    None => Err("Not Found".to_string())
                }
            }
            _ => return Err("Command not valid".to_string()),
        }
    }
}

fn main() {
    let redis: Redis = Redis::new();

    let set = redis.execute(&Command::Key(3), &"hola".to_string(), &"chau".to_string());
    let get = redis.execute(&Command::Key(2), &"hola".to_string(), &"".to_string());

    println!("Hello, world!");
}
