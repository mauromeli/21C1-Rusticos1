use std::collections::HashMap;

#[derive(Debug)]
pub struct Redis {
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

    pub fn execute(&mut self, command: &Command, params: Vec<&String>) -> Result<String, String> {
        match command {
            Command::Key(ref command) if command == "PING" => {
                Ok("PONG".to_string())
            }
            Command::Key(ref command) if command == "get" => {
                self.get_method(params)
            }
            Command::Key(ref command) if command == "set" => {
                self.set_method(params)
            }
            _ => return Err("Command not valid".to_string()),
        }
    }

    fn get_method(&mut self, params: Vec<&String>) -> Result<String, String> {
        if params.len() != 1 {
            return Err("ERR wrong number of arguments for 'get' command".to_string());
        }
        match self.db.get(params[0].as_str()) {
            Some(return_value) => Ok(return_value.to_string()),
            None => Err("Not Found".to_string())
        }
    }

    fn set_method(&mut self, params: Vec<&String>) -> Result<String, String> {
        if params.len() != 2 {
            return Err("ERR syntax error".to_string());
        }
        match self.db.insert(params[0].to_string(), params[1].to_string()) {
            Some(_) => Ok("Ok".to_string()),
            None => Err("Not Found".to_string())
        }
    }
}

#[test]
fn test_set_element_and_get_the_same() {
    let mut redis: Redis = Redis::new();

    let value: String = "chau".to_string();
    let key: String = "hola".to_string();
    let params_set = vec![&key, &value];
    let params_get = vec![&key];

    let _set = redis.execute(&Command::Key("set".to_string()), params_set);

    let get: Result<String, String> = redis.execute(&Command::Key("get".to_string()), params_get);

    assert_eq!(value, get.unwrap().to_string());
}

#[test]
fn test_set_element_twice_and_get_the_last_set() {
    let mut redis: Redis = Redis::new();

    let key: String = "hola".to_string();
    let value: String = "chau".to_string();
    let second_value: String = "test".to_string();

    let params_first_set = vec![&key, &value];
    let params_second_set = vec![&key, &second_value];
    let params_get = vec![&key];

    let _set = redis.execute(&Command::Key("set".to_string()), params_first_set);
    let _set = redis.execute(&Command::Key("set".to_string()), params_second_set);

    let get: Result<String, String> = redis.execute(&Command::Key("get".to_string()), params_get);

    assert_eq!(second_value, get.unwrap().to_string());
}

#[test]
fn test_get_element_not_found() {
    let mut redis: Redis = Redis::new();

    let key = "hola".to_string();
    let params_get = vec![&key];

    let get: Result<String, String> = redis.execute(&Command::Key("get".to_string()), params_get);
    assert!(get.is_err());
}

#[test]
fn test_get_params_err() {
    let mut redis: Redis = Redis::new();

    let key = "hola".to_string();
    let another_param = "hola".to_string();
    let params_get = vec![&key, &another_param];

    let get: Result<String, String> = redis.execute(&Command::Key("get".to_string()), params_get);
    assert!(get.is_err());
}

#[test]
fn test_get_without_params_err() {
    let mut redis: Redis = Redis::new();

    let params_get = vec![];

    let get: Result<String, String> = redis.execute(&Command::Key("get".to_string()), params_get);
    assert!(get.is_err());
}

#[test]
fn test_set_params_err() {
    let mut redis: Redis = Redis::new();

    let key = "hola".to_string();
    let value = "value".to_string();
    let another_param = "hola".to_string();
    let params_set = vec![&key, &value, &another_param];

    let set: Result<String, String> = redis.execute(&Command::Key("get".to_string()), params_set);
    assert!(set.is_err());
}

#[test]
fn test_set_without_params_err() {
    let mut redis: Redis = Redis::new();

    let params_set = vec![];

    let set: Result<String, String> = redis.execute(&Command::Key("set".to_string()), params_set);
    assert!(set.is_err());
}

#[test]
fn test_method_not_found() {
    let mut redis: Redis = Redis::new();

    let key: String = "hola".to_string();
    let params = vec![&key];

    let method_not_valid: Result<String, String> = redis.execute(&Command::Key("method".to_string()), params);
    assert!(method_not_valid.is_err());
}

#[test]
fn test_ping_retunrs_pong() {
    let mut redis: Redis = Redis::new();

    let pong: String = "PONG".to_string();

    let ping: Result<String, String> = redis.execute(&Command::Key("PING".to_string()), vec![]);
    assert_eq!(pong, ping.unwrap().to_string());
}