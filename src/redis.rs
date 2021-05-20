use std::collections::HashMap;

#[derive(Debug)]
struct Redis {
    db: HashMap<String, String>,
}

#[derive(Debug)]
pub enum Command {
    Key(String)
}

impl Redis {
    pub fn new() -> Self {
        let map: HashMap<String, String> = HashMap::new();

        return Self {
            db: map
        };
    }

    pub fn execute(&mut self, command: &Command, key: &String, value: &String) -> Result<String, String> {
        match command {
            Command::Key(ref command) if command == "PING" => {
                Ok("PONG".to_string())
            }
            Command::Key(ref command) if command == "get" => {
                match self.db.get(key) {
                    Some(return_value) => Ok(return_value.to_string()),
                    None => Err("Not Found".to_string())
                }
            }
            Command::Key(ref command) if command == "set" => {
                match self.db.insert(key.to_string(), value.to_string()) {
                    Some(_) => Ok("Saved".to_string()),
                    None => Err("Not Found".to_string())
                }
            }
            _ => return Err("Command not valid".to_string()),
        }
    }
}

#[test]
fn test_set_element_and_get_the_same() {
    let mut redis: Redis = Redis::new();

    let value: String = "chau".to_string();
    let _set = redis.execute(&Command::Key("set".to_string()), &"hola".to_string(), &value);

    let get: Result<String, String> = redis.execute(&Command::Key("get".to_string()), &"hola".to_string(), &"".to_string());

    assert_eq!(value, get.unwrap().to_string());
}

#[test]
fn test_get_element_not_found() {
    let mut redis: Redis = Redis::new();

    let get: Result<String, String> = redis.execute(&Command::Key("get".to_string()), &"hola".to_string(), &"".to_string());
    assert!(get.is_err());
}

#[test]
fn test_ping_retunrs_pong() {
    let mut redis: Redis = Redis::new();

    let pong: String = "PONG".to_string();

    let ping: Result<String, String> = redis.execute(&Command::Key("PING".to_string()), &"hola".to_string(), &"".to_string());
    assert_eq!(pong, ping.unwrap().to_string());
}