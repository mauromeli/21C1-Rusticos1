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

    pub fn execute(&mut self, command: &Command, key: &String, value: &String) -> Result<String, String> {
        match command {
            Command::Key(ref command) if command == "PING" => {
                Ok("PONG".to_string())
            }
            Command::Key(ref command) if command == "get" => {
                self.get_method(key)
            }
            Command::Key(ref command) if command == "set" => {
                self.set_method(key, value)
            }
            Command::Key(ref command) if command == "append" => {
                self.append_method(key, value)
            }
            _ => return Err("Command not valid".to_string()),
        }
    }

    fn get_method(&mut self, key: &String) -> Result<String, String> {
        match self.db.get(key) {
            Some(return_value) => Ok(return_value.to_string()),
            None => Err("Not Found".to_string())
        }
    }

    fn set_method(&mut self, key: &String, value: &String) -> Result<String, String> {
        match self.db.insert(key.to_string(), value.to_string()) {
            Some(_) => Ok("Ok".to_string()),
            None => Err("Not Found".to_string())
        }
    }

    fn append_method(&mut self, key: &String, value: &String) -> Result<String, String> {
        /*let key_value = key.clone();
        let get_value = self.db.get(&key_value.to_string());
        if get_value.is_some() {
            /*let mut val = get_value.unwrap().to_string();
            match self.db.insert(key.to_string(), val.push_str(value)) {
                Some(value) => Ok(value.len().to_string()),
                None => Err("Not Found".to_string())
            }*/
            return Ok("Estaba Guardada".to_string())
        }*/

        /*match self.db.insert(key.to_string(), value.to_string()) {
            Some(response) => Ok(response.len().to_string()),
            None => Err("Not Found".to_string())
        }*/
        match self.db.insert(key.to_string(), value.to_string()) {
            Some(_) => Ok("value.len()".to_string()),
            None => Err("Not Found".to_string())
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
fn test_method_not_found() {
    let mut redis: Redis = Redis::new();

    let method_not_valid: Result<String, String> = redis.execute(&Command::Key("method".to_string()), &"hola".to_string(), &"".to_string());
    assert!(method_not_valid.is_err());
}

#[test]
fn test_ping_retunrs_pong() {
    let mut redis: Redis = Redis::new();

    let pong: String = "PONG".to_string();

    let ping: Result<String, String> = redis.execute(&Command::Key("PING".to_string()), &"hola".to_string(), &"".to_string());
    assert_eq!(pong, ping.unwrap().to_string());
}

/*
#[test]
fn test_set_element_and_append() {
    let mut redis: Redis = Redis::new();

    let value: String = "hola".to_string();
    let _set = redis.execute(&Command::Key("set".to_string()), &"key".to_string(), &value);
    let append_response = redis.execute(&Command::Key("append".to_string()), &"key".to_string(), &"chau".to_string());


    let get: Result<String, String> = redis.execute(&Command::Key("get".to_string()), &"key".to_string(), &"".to_string());

    assert_eq!("holachau".to_string(), get.unwrap().to_string());
    assert_eq!("8".to_string(), append_response.unwrap());
}

#[test]
fn test_append_element_without_set() {
    let mut redis: Redis = Redis::new();

    let value: String = "chau".to_string();
    let _set = redis.execute(&Command::Key("append".to_string()), &"hola".to_string(), &value);

    let get: Result<String, String> = redis.execute(&Command::Key("get".to_string()), &"hola".to_string(), &"".to_string());

    assert_eq!(value.len().to_string(), get.unwrap().to_string());
}
*/