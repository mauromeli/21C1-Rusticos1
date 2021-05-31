use crate::entities::command::Command;
use crate::entities::redis_element::RedisElement;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Redis {
    db: HashMap<String, RedisElement>,
}

impl Redis {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let map = HashMap::new();

        Self { db: map }
    }

    #[allow(dead_code)]
    pub fn execute(&mut self, command: Command, params: Vec<&String>) -> Result<String, String> {
        match command {
            Command::Key(ref command) if command == "PING" => Ok("PONG".to_string()),
            Command::Key(ref command) if command == "copy" => self.copy_method(params),
            Command::Key(ref command) if command == "get" => self.get_method(params),
            Command::Key(ref command) if command == "set" => self.set_method(params),
            Command::Key(ref command) if command == "del" => self.del_method(params),
            Command::Key(ref command) if command == "exists" => self.exists_method(params),
            Command::Key(ref command) if command == "rename" => self.rename_method(params),
            Command::Key(ref command) if command == "incrby" => self.incrby_method(params),
            Command::Key(ref command) if command == "getdel" => self.getdel_method(params),
            Command::Key(ref command) if command == "append" => self.append_method(params),
            Command::Key(ref command) if command == "dbsize" => Ok(self.db.len().to_string()),
            _ => Err("Command not valid".to_string()),
        }
    }

    #[allow(dead_code)]
    fn copy_method(&mut self, params: Vec<&String>) -> Result<String, String> {
        // TODO: no debería usar el metodo SET, si se estan copiando valores deberia mantenerse el tipo de elemento (String, Set, List)
        if params.len() != 2 {
            return Err("ERR wrong number of arguments for 'copy' command".to_string());
        }
        match self.get_method(vec![params[0]]) {
            Ok(value) => self.set_method(vec![params[1], &value]),
            Err(_) => Err("Not Found".to_string()),
        }
    }

    #[allow(dead_code)]
    fn get_method(&mut self, params: Vec<&String>) -> Result<String, String> {
        // TODO: deberia devolver NIL si no existe el elemento
        if params.len() != 1 {
            return Err("ERR wrong number of arguments for 'get' command".to_string());
        }
        match self.db.get(params[0].as_str()) {
            Some(return_value) => match return_value {
                RedisElement::String(_) => Ok(return_value.to_string()),
                _ => Err("Not string".to_string()),
            },
            None => Err("Not Found".to_string()),
        }
    }

    #[allow(dead_code)]
    fn set_method(&mut self, params: Vec<&String>) -> Result<String, String> {
        if params.len() != 2 {
            return Err("ERR syntax error".to_string());
        }
        self.db.insert(
            params[0].to_string(),
            RedisElement::String(params[1].to_string()),
        );
        Ok("Ok".to_string())
    }

    #[allow(dead_code)]
    fn incrby_method(&mut self, params: Vec<&String>) -> Result<String, String> {
        let incr_value: Result<u32, _> = params[1].to_string().parse();

        if params.len() != 2 || incr_value.is_err() {
            return Err("ERR syntax error".to_string());
        }
        match self.get_method(vec![params[0]]) {
            Ok(return_value) => {
                let my_int: Result<u32, _> = return_value.parse();
                if my_int.is_err() {
                    return Err("ERR value is not an integer or out of range".to_string());
                }

                self.set_method(vec![
                    params[0],
                    &(my_int.unwrap() + incr_value.unwrap()).to_string(),
                ])
            }
            Err(_) => self.set_method(params),
        }
    }

    #[allow(dead_code)]
    fn getdel_method(&mut self, params: Vec<&String>) -> Result<String, String> {
        if params.len() != 1 {
            return Err("ERR wrong number of arguments for 'getdel' command".to_string());
        }
        match self.get_method(params.clone()) {
            Ok(return_value) => {
                self.db.remove(params[0].as_str());
                Ok(return_value)
            }
            Err(_) => Err("Not Found".to_string()),
        }
    }

    #[allow(dead_code)]
    fn del_method(&mut self, params: Vec<&String>) -> Result<String, String> {
        if params.is_empty() {
            return Err("ERR wrong number of arguments for 'del' command".to_string());
        }
        let mut count = 0;
        for param in params.iter() {
            if self.db.remove(param.as_str()).is_some() {
                count += 1;
            }
        }
        Ok(count.to_string())
    }

    #[allow(dead_code)]
    fn append_method(&mut self, params: Vec<&String>) -> Result<String, String> {
        //TODO: chequar si el valor es string antes de hacer el append
        if params.len() != 2 {
            return Err("ERR wrong number of arguments for 'append' command".to_string());
        }
        match self.get_method(vec![params[0]]) {
            Ok(return_value) => {
                let value = return_value + params[1];
                self.set_method(vec![params[0], &value])
            }
            Err(_) => self.set_method(params),
        }
    }

    fn exists_method(&mut self, params: Vec<&String>) -> Result<String, String> {
        if params.is_empty() {
            return Err("ERR wrong number of arguments for 'exists' command".to_string());
        }

        let mut count = 0;
        for param in params.iter() {
            if self.db.contains_key(param.as_str()) {
                count += 1;
            }
        }
        Ok(count.to_string())
    }

    fn rename_method(&mut self, params: Vec<&String>) -> Result<String, String> {
        if params.len() != 2 {
            return Err("ERR wrong number of arguments for 'exists' command".to_string());
        }

        match self.getdel_method(vec![params[0]]) {
            Ok(value) => self.set_method(vec![params[1], &value]),
            Err(msg) => Err(msg),
        }
    }
}

#[allow(unused_imports)]
mod test {
    use crate::entities::command::Command;
    use crate::service::redis::Redis;

    #[test]
    fn test_set_element_and_get_the_same() {
        let mut redis: Redis = Redis::new();

        let value: String = "chau".to_string();
        let key: String = "hola".to_string();
        let params_set = vec![&key, &value];
        let params_get = vec![&key];

        let _set = redis.execute(Command::Key("set".to_string()), params_set);

        let get: Result<String, String> =
            redis.execute(Command::Key("get".to_string()), params_get);

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

        let _set = redis.execute(Command::Key("set".to_string()), params_first_set);
        let _set = redis.execute(Command::Key("set".to_string()), params_second_set);

        let get: Result<String, String> =
            redis.execute(Command::Key("get".to_string()), params_get);

        assert_eq!(second_value, get.unwrap().to_string());
    }

    #[test]
    fn test_get_element_not_found() {
        let mut redis: Redis = Redis::new();

        let key = "hola".to_string();
        let params_get = vec![&key];

        let get: Result<String, String> =
            redis.execute(Command::Key("get".to_string()), params_get);
        assert!(get.is_err());
    }

    #[test]
    fn test_get_params_err() {
        let mut redis: Redis = Redis::new();

        let key = "hola".to_string();
        let another_param = "hola".to_string();
        let params_get = vec![&key, &another_param];

        let get: Result<String, String> =
            redis.execute(Command::Key("get".to_string()), params_get);
        assert!(get.is_err());
    }

    #[test]
    fn test_get_without_params_err() {
        let mut redis: Redis = Redis::new();

        let params_get = vec![];

        let get: Result<String, String> =
            redis.execute(Command::Key("get".to_string()), params_get);
        assert!(get.is_err());
    }

    #[test]
    fn test_set_params_err() {
        let mut redis: Redis = Redis::new();

        let key = "hola".to_string();
        let value = "value".to_string();
        let another_param = "hola".to_string();
        let params_set = vec![&key, &value, &another_param];

        let set: Result<String, String> =
            redis.execute(Command::Key("get".to_string()), params_set);
        assert!(set.is_err());
    }

    #[test]
    fn test_set_without_params_err() {
        let mut redis: Redis = Redis::new();

        let params_set = vec![];

        let set: Result<String, String> =
            redis.execute(Command::Key("set".to_string()), params_set);
        assert!(set.is_err());
    }

    #[test]
    fn test_method_not_found() {
        let mut redis: Redis = Redis::new();

        let key: String = "hola".to_string();
        let params = vec![&key];

        let method_not_valid: Result<String, String> =
            redis.execute(Command::Key("method".to_string()), params);
        assert!(method_not_valid.is_err());
    }

    #[test]
    fn test_ping_retunrs_pong() {
        let mut redis: Redis = Redis::new();

        let pong: String = "PONG".to_string();

        let ping: Result<String, String> = redis.execute(Command::Key("PING".to_string()), vec![]);
        assert_eq!(pong, ping.unwrap().to_string());
    }

    #[test]
    fn test_incrby_with_2_as_value() {
        let mut redis: Redis = Redis::new();

        let key: String = "key".to_string();
        let value: String = "1".to_string();
        let increment: String = "1".to_string();
        let second_increment: String = "2".to_string();

        let params_set = vec![&key, &value];
        let params_incrby = vec![&key, &increment];
        let params_second_incrby = vec![&key, &second_increment];

        let params_get = vec![&key];

        let _set = redis.execute(Command::Key("set".to_string()), params_set);
        let _incrby = redis.execute(Command::Key("incrby".to_string()), params_incrby);

        let get: Result<String, String> =
            redis.execute(Command::Key("get".to_string()), params_get.clone());

        let _incrby = redis.execute(Command::Key("incrby".to_string()), params_second_incrby);
        let second_get: Result<String, String> =
            redis.execute(Command::Key("get".to_string()), params_get);

        assert_eq!("2".to_string(), get.unwrap().to_string());
        assert_eq!("4".to_string(), second_get.clone().unwrap().to_string());
        assert_ne!("10".to_string(), second_get.unwrap().to_string());
    }

    #[test]
    fn test_incrby_value_err_initial_value_string() {
        let mut redis: Redis = Redis::new();

        let key: String = "key".to_string();
        let value: String = "hola".to_string();
        let increment: String = "1".to_string();

        let params_set = vec![&key, &value];
        let params_incrby = vec![&key, &increment];

        let _set = redis.execute(Command::Key("set".to_string()), params_set);
        let incrby = redis.execute(Command::Key("incrby".to_string()), params_incrby);

        assert!(incrby.is_err());
    }

    #[test]
    fn test_incrby_not_saved_value() {
        let mut redis: Redis = Redis::new();

        let key: String = "key".to_string();
        let increment: String = "1".to_string();

        let params_incrby = vec![&key, &increment];
        let params_get = vec![&key];

        let _incrby = redis.execute(Command::Key("incrby".to_string()), params_incrby);

        let get: Result<String, String> =
            redis.execute(Command::Key("get".to_string()), params_get.clone());
        let second_get: Result<String, String> =
            redis.execute(Command::Key("get".to_string()), params_get);

        assert_eq!("1".to_string(), get.unwrap().to_string());
        assert_ne!("10".to_string(), second_get.unwrap().to_string());
    }

    #[test]
    fn test_set_element_and_getdel() {
        let mut redis: Redis = Redis::new();

        let value: String = "value".to_string();
        let key: String = "key".to_string();
        let params_set = vec![&key, &value];
        let params_get = vec![&key];

        let _set = redis.execute(Command::Key("set".to_string()), params_set);

        let get: Result<String, String> =
            redis.execute(Command::Key("get".to_string()), params_get.clone());
        let getdel: Result<String, String> =
            redis.execute(Command::Key("getdel".to_string()), params_get.clone());

        assert_eq!(value, get.unwrap().to_string());
        assert_eq!(value, getdel.unwrap().to_string());

        let get: Result<String, String> =
            redis.execute(Command::Key("get".to_string()), params_get);
        assert!(get.is_err());
    }

    #[test]
    fn test_getdel_without_params_err() {
        let mut redis: Redis = Redis::new();

        let params_getdel = vec![];

        let getdel: Result<String, String> =
            redis.execute(Command::Key("getdel".to_string()), params_getdel);
        assert!(getdel.is_err());
    }

    #[test]
    fn test_getdel_without_previews_saving_err() {
        let mut redis: Redis = Redis::new();

        let key: String = "key".to_string();
        let params_getdel = vec![&key];

        let getdel: Result<String, String> =
            redis.execute(Command::Key("getdel".to_string()), params_getdel);
        assert!(getdel.is_err());
    }

    #[test]
    fn test_dbsize() {
        let mut redis: Redis = Redis::new();

        let dbsize: Result<String, String> =
            redis.execute(Command::Key("dbsize".to_string()), vec![]);

        assert_eq!("0".to_string(), dbsize.unwrap().to_string());

        let value: String = "value".to_string();
        let key: String = "key".to_string();
        let params_set = vec![&key, &value];

        let _set = redis.execute(Command::Key("set".to_string()), params_set);
        let dbsize: Result<String, String> =
            redis.execute(Command::Key("dbsize".to_string()), vec![]);

        assert_eq!("1".to_string(), dbsize.unwrap().to_string());

        let params_get = vec![&key];
        let _getdel: Result<String, String> =
            redis.execute(Command::Key("getdel".to_string()), params_get);

        let dbsize: Result<String, String> =
            redis.execute(Command::Key("dbsize".to_string()), vec![]);

        assert_eq!("0".to_string(), dbsize.unwrap().to_string());
    }

    #[test]
    fn test_set_element_and_del() {
        let mut redis: Redis = Redis::new();

        let value: String = "value".to_string();
        let key: String = "key".to_string();
        let params_set = vec![&key, &value];
        let params_del = vec![&key];
        let params_get = vec![&key];

        let _set = redis.execute(Command::Key("set".to_string()), params_set);
        let del: Result<String, String> =
            redis.execute(Command::Key("del".to_string()), params_del);

        assert_eq!("1".to_string(), del.unwrap().to_string());

        let get: Result<String, String> =
            redis.execute(Command::Key("get".to_string()), params_get);
        assert!(get.is_err());
    }

    #[test]
    fn test_set_two_elements_and_del_both() {
        let mut redis: Redis = Redis::new();

        let value: String = "value".to_string();
        let key1: String = "key1".to_string();
        let key2: String = "key2".to_string();
        let params_set1 = vec![&key1, &value];
        let params_set2 = vec![&key2, &value];
        let params_del = vec![&key1, &key2];

        let _set = redis.execute(Command::Key("set".to_string()), params_set1);
        let _set = redis.execute(Command::Key("set".to_string()), params_set2);

        let del: Result<String, String> =
            redis.execute(Command::Key("del".to_string()), params_del);

        assert_eq!("2".to_string(), del.unwrap().to_string());
    }

    #[test]
    fn test_del_without_params_err() {
        let mut redis: Redis = Redis::new();

        let params_del = vec![];

        let del: Result<String, String> =
            redis.execute(Command::Key("del".to_string()), params_del);
        assert!(del.is_err());
    }

    #[test]
    fn test_append_adds_word() {
        let mut redis: Redis = Redis::new();

        let value: String = "value".to_string();
        let value_append: String = " appended".to_string();
        let key: String = "key".to_string();
        let params_set = vec![&key, &value];
        let params_append = vec![&key, &value_append];
        let params_get = vec![&key];

        let _set = redis.execute(Command::Key("set".to_string()), params_set);
        let _append = redis.execute(Command::Key("append".to_string()), params_append);

        let get: Result<String, String> =
            redis.execute(Command::Key("get".to_string()), params_get);

        assert_eq!("value appended".to_string(), get.unwrap());
    }

    #[test]
    fn test_append_on_non_existent_key() {
        let mut redis: Redis = Redis::new();

        let value_append: String = "appended".to_string();
        let key: String = "key".to_string();
        let params_append = vec![&key, &value_append];
        let params_get = vec![&key];

        let _append = redis.execute(Command::Key("append".to_string()), params_append);

        let get: Result<String, String> =
            redis.execute(Command::Key("get".to_string()), params_get);

        assert_eq!("appended".to_string(), get.unwrap());
    }

    #[test]
    fn test_append_whithout_params_err() {
        let mut redis: Redis = Redis::new();

        let key: String = "key".to_string();
        let params_append = vec![&key];
        let params_get = vec![&key];

        let _append = redis.execute(Command::Key("append".to_string()), params_append);

        let get: Result<String, String> =
            redis.execute(Command::Key("get".to_string()), params_get);

        assert!(get.is_err());
    }

    #[test]
    fn test_set_two_elements_and_check_exists_equal_2() {
        let mut redis: Redis = Redis::new();

        let value: String = "value".to_string();
        let key1: String = "key1".to_string();
        let key2: String = "key2".to_string();
        let key3: String = "key3".to_string();

        let params_set1 = vec![&key1, &value];
        let params_set2 = vec![&key2, &value];
        let params_exists = vec![&key1, &key2];
        let params_exists_2 = vec![&key1, &key2, &key3];

        let _set = redis.execute(Command::Key("set".to_string()), params_set1);
        let _set = redis.execute(Command::Key("set".to_string()), params_set2);

        let exists: Result<String, String> =
            redis.execute(Command::Key("exists".to_string()), params_exists);

        assert_eq!("2".to_string(), exists.unwrap().to_string());

        let exists: Result<String, String> =
            redis.execute(Command::Key("exists".to_string()), params_exists_2);

        assert_eq!("2".to_string(), exists.unwrap().to_string());
    }

    #[test]
    fn test_exists_without_params_err() {
        let mut redis: Redis = Redis::new();

        let params_exists = vec![];

        let exists: Result<String, String> =
            redis.execute(Command::Key("exists".to_string()), params_exists);
        assert!(exists.is_err());
    }

    #[test]
    fn test_copy_without_params_err() {
        let mut redis: Redis = Redis::new();

        let params_copy = vec![];

        let copy: Result<String, String> =
            redis.execute(Command::Key("copy".to_string()), params_copy);
        assert!(copy.is_err());
    }

    #[test]
    fn test_set_two_elements_and_copy() {
        let mut redis: Redis = Redis::new();

        let value1: String = "value1".to_string();
        let value2: String = "value2".to_string();
        let key1: String = "key1".to_string();
        let key2: String = "key2".to_string();
        let params_set1 = vec![&key1, &value1];
        let params_set2 = vec![&key2, &value2];
        let params_copy = vec![&key1, &key2];
        let params_get = vec![&key2];

        let _set = redis.execute(Command::Key("set".to_string()), params_set1);
        let _set = redis.execute(Command::Key("set".to_string()), params_set2);

        let get = redis.execute(Command::Key("get".to_string()), params_get.clone());

        assert_eq!("value2".to_string(), get.unwrap().to_string());
        let _copy = redis.execute(Command::Key("copy".to_string()), params_copy);
        let get = redis.execute(Command::Key("get".to_string()), params_get);
        assert_eq!("value1".to_string(), get.unwrap().to_string());
    }

    #[test]
    fn test_rename_without_params_err() {
        let mut redis: Redis = Redis::new();

        let params = vec![];

        let rename: Result<String, String> =
            redis.execute(Command::Key("rename".to_string()), params);
        assert!(rename.is_err());
    }

    #[test]
    fn test_set_and_rename() {
        let mut redis: Redis = Redis::new();

        let value1: String = "value1".to_string();
        let key1: String = "key1".to_string();
        let key2: String = "key2".to_string();
        let params_set1 = vec![&key1, &value1];
        let params_rename = vec![&key1, &key2];
        let params_first_get = vec![&key1];
        let params_second_get = vec![&key2];

        let _set = redis.execute(Command::Key("set".to_string()), params_set1);
        let rename = redis.execute(Command::Key("rename".to_string()), params_rename);

        assert!(rename.is_ok());
        let get = redis.execute(Command::Key("get".to_string()), params_first_get);
        assert!(get.is_err());

        let get = redis.execute(Command::Key("get".to_string()), params_second_get);
        assert!(get.is_ok());
        assert_eq!("value1".to_string(), get.unwrap().to_string());
    }
}
