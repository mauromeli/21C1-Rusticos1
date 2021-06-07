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
    pub fn execute(&mut self, command: Command) -> Result<RedisElement, String> {
        match command {
            // Server
            Command::Dbsize => Ok(RedisElement::String(self.db.len().to_string())),
            Command::Ping => Ok(RedisElement::String("PONG".to_string())),

            // Strings
            Command::Append { key, value } => self.append_method(key, value),
            Command::Decrby { key, decrement } => self.incrby_method(key, -(decrement as i32)),
            Command::Get { key } => self.get_method(key),
            Command::Getdel { key } => self.getdel_method(key),
            Command::Getset { key, value } => self.getset_method(key, value),
            Command::Incrby { key, increment } => self.incrby_method(key, increment as i32),
            Command::Mget { keys } => Ok(self.mget_method(keys)),
            Command::Mset { key_values } => Ok(RedisElement::String(self.mset_method(key_values))),
            Command::Set { key, value } => Ok(RedisElement::String(self.set_method(key, value))),

            // Keys
            Command::Copy {
                key_origin,
                key_destination,
            } => self.copy_method(key_origin, key_destination),
            Command::Del { keys } => Ok(RedisElement::String(self.del_method(keys))),
            Command::Exists { keys } => Ok(RedisElement::String(self.exists_method(keys))),
            Command::Rename {
                key_origin,
                key_destination,
            } => self.rename_method(key_origin, key_destination),

            // Lists
            Command::Lindex { key, index } => self.lindex_method(key, index),
            Command::Llen { key } => self.llen_method(key),
            Command::Lpush { key, value } => self.lpush_method(key, value),
        }
    }

    #[allow(dead_code)]
    fn copy_method(
        &mut self,
        key_origin: String,
        key_destination: String,
    ) -> Result<RedisElement, String> {
        //TODO: deberia devolver 1 si se copio y 0 si no!
        match self.db.get(key_origin.as_str()) {
            Some(value) => match self.db.get(key_destination.as_str()) {
                Some(_) => Err("ERR destination key already holds a value".to_string()),
                None => {
                    let value = value.clone();
                    self.db.insert(key_destination, value);
                    Ok(RedisElement::String("1".to_string()))
                }
            },
            None => Err("ERR origin key has no value".to_string()),
        }
    }

    #[allow(dead_code)]
    fn get_method(&mut self, key: String) -> Result<RedisElement, String> {
        match self.db.get(key.as_str()) {
            Some(return_value) => match return_value {
                RedisElement::String(s) => Ok(RedisElement::String(s.to_string())),
                _ => Err(
                    "WRONGTYPE Operation against a key holding the wrong kind of value".to_string(),
                ),
            },
            None => Ok(RedisElement::Nil),
        }
    }

    #[allow(dead_code)]
    fn getset_method(&mut self, key: String, value: String) -> Result<RedisElement, String> {
        //TODO: revisar que tiene que setear aunque get devuelva nil. Agregar tests.
        match self.get_method(key.clone()) {
            Ok(return_value) => {
                self.set_method(key, value);
                Ok(RedisElement::String(return_value.to_string()))
            }
            Err(e) => Err(e),
        }
    }

    #[allow(dead_code)]
    fn set_method(&mut self, key: String, value: String) -> String {
        self.db.insert(key, RedisElement::String(value));

        "Ok".to_string()
    }

    #[allow(dead_code)]
    fn incrby_method(&mut self, key: String, increment: i32) -> Result<RedisElement, String> {
        match self.get_method(key.clone()) {
            Ok(return_value) => match return_value {
                RedisElement::String(value) => {
                    let my_int: Result<i32, _> = value.parse();
                    if my_int.is_err() {
                        return Err("ERR value is not an integer or out of range".to_string());
                    }

                    let my_int = my_int.unwrap() + increment;
                    Ok(RedisElement::String(
                        self.set_method(key, my_int.to_string()),
                    ))
                }
                RedisElement::Nil => Ok(RedisElement::String(
                    self.set_method(key, increment.to_string()),
                )),
                _ => Err(
                    "WRONGTYPE Operation against a key holding the wrong kind of value".to_string(),
                ),
            },
            Err(_) => Ok(RedisElement::String(
                self.set_method(key, increment.to_string()),
            )),
        }
    }

    #[allow(dead_code)]
    fn mget_method(&mut self, keys: Vec<String>) -> RedisElement {
        //TODO: tests
        let mut elements: Vec<String> = Vec::new();
        for key in keys.iter() {
            elements.push(self.get_method(key.to_string()).unwrap().to_string());
        }
        RedisElement::List(elements)
    }

    #[allow(dead_code)]
    fn mset_method(&mut self, key_values: Vec<(String, String)>) -> String {
        for (key, value) in key_values.iter() {
            self.set_method(key.to_string(), value.to_string());
        }
        "Ok".to_string()
    }

    #[allow(dead_code)]
    fn getdel_method(&mut self, key: String) -> Result<RedisElement, String> {
        match self.get_method(key.clone()) {
            Ok(return_value) => match return_value {
                RedisElement::String(_) => {
                    self.db.remove(key.as_str());
                    Ok(return_value)
                }
                RedisElement::Nil => Ok(return_value),
                _ => Err(
                    "WRONGTYPE Operation against a key holding the wrong kind of value".to_string(),
                ),
            },
            Err(msg) => Err(msg),
        }
    }

    #[allow(dead_code)]
    fn del_method(&mut self, keys: Vec<String>) -> String {
        let mut count = 0;
        for key in keys.iter() {
            if self.db.remove(key.as_str()).is_some() {
                count += 1;
            }
        }

        count.to_string()
    }

    #[allow(dead_code)]
    fn append_method(&mut self, key: String, value: String) -> Result<RedisElement, String> {
        match self.get_method(key.clone()) {
            Ok(redis_element) => match redis_element {
                RedisElement::String(s) => {
                    let value = s + value.as_str();
                    Ok(RedisElement::String(self.set_method(key, value)))
                }
                RedisElement::Nil => Ok(RedisElement::String(self.set_method(key, value))),
                _ => Err(
                    "WRONGTYPE Operation against a key holding the wrong kind of value".to_string(),
                ),
            },
            Err(e) => Err(e),
        }
    }

    fn exists_method(&mut self, keys: Vec<String>) -> String {
        let mut count = 0;
        for key in keys.iter() {
            if self.db.contains_key(key.as_str()) {
                count += 1;
            }
        }
        count.to_string()
    }

    fn rename_method(
        &mut self,
        key_origin: String,
        key_destination: String,
    ) -> Result<RedisElement, String> {
        match self.getdel_method(key_origin) {
            Ok(value) => Ok(RedisElement::String(
                self.set_method(key_destination, value.to_string()),
            )),
            Err(msg) => Err(msg),
        }
    }

    fn lindex_method(&mut self, key: String, index: i32) -> Result<RedisElement, String> {
        match self.db.get_mut(key.as_str()) {
            Some(value) => match value {
                RedisElement::List(value) => {
                    let len_value = value.len() as i32;
                    let mut position: i32 = index;

                    if index < 0 {
                        position = index + len_value;
                    }

                    match value.get(position as usize) {
                        Some(saved_value) => Ok(RedisElement::String(saved_value.to_string())),
                        None => Ok(RedisElement::Nil),
                    }
                }
                _ => Err(
                    "WRONGTYPE Operation against a key holding the wrong kind of value".to_string(),
                ),
            },
            None => Ok(RedisElement::Nil),
        }
    }

    fn llen_method(&mut self, key: String) -> Result<RedisElement, String> {
        match self.db.get_mut(key.as_str()) {
            Some(value) => match value {
                RedisElement::List(value) => Ok(RedisElement::String(value.len().to_string())),
                _ => Err(
                    "WRONGTYPE Operation against a key holding the wrong kind of value".to_string(),
                ),
            },
            None => Ok(RedisElement::String("0".to_string())),
        }
    }

    fn lpush_method(&mut self, key: String, values: Vec<String>) -> Result<RedisElement, String> {
        let mut redis_element: Vec<String> = values;
        redis_element.reverse();

        match self.db.get_mut(key.as_str()) {
            Some(value) => match value {
                RedisElement::List(value) => {
                    let saved_vector = value.clone();
                    redis_element.extend(saved_vector);
                    self.db
                        .insert(key, RedisElement::List(redis_element.clone()));

                    Ok(RedisElement::String(redis_element.len().to_string()))
                }
                _ => Err(
                    "WRONGTYPE Operation against a key holding the wrong kind of value".to_string(),
                ),
            },
            None => {
                self.db
                    .insert(key, RedisElement::List(redis_element.clone()));

                Ok(RedisElement::String(redis_element.len().to_string()))
            }
        }
    }
}

#[allow(unused_imports)]
mod test {
    use crate::entities::command::Command;
    use crate::service::redis::Redis;
    use crate::service::redis::RedisElement;
    #[allow(unused_imports)]
    #[test]
    fn test_set_element_and_get_the_same() {
        let mut redis: Redis = Redis::new();

        let value: String = "value".to_string();
        let key: String = "hola".to_string();

        let _set = redis.execute(Command::Set { key, value });

        let key: String = "hola".to_string();
        let get: Result<RedisElement, String> = redis.execute(Command::Get { key });

        assert_eq!("value".to_string(), get.unwrap().to_string());
    }

    #[test]
    fn test_set_element_twice_and_get_the_last_set() {
        let mut redis: Redis = Redis::new();

        let key: String = "hola".to_string();
        let value: String = "chau".to_string();

        let _set = redis.execute(Command::Set { key, value });

        let key: String = "hola".to_string();
        let value: String = "test".to_string();
        let _set = redis.execute(Command::Set { key, value });

        let key: String = "hola".to_string();
        let get: Result<RedisElement, String> = redis.execute(Command::Get { key });

        assert_eq!("test".to_string(), get.unwrap().to_string());
    }

    #[test]
    fn test_get_element_not_found() {
        let mut redis: Redis = Redis::new();

        let key = "hola".to_string();
        let get: Result<RedisElement, String> = redis.execute(Command::Get { key });

        assert_eq!("(nil)", get.unwrap().to_string());
    }

    #[test]
    fn test_get_element_fail_if_is_not_string() {
        let mut redis: Redis = Redis::new();

        let key: String = "key".to_string();
        let value = vec!["value".to_string(), "value2".to_string()];
        let _lpush = redis.execute(Command::Lpush { key, value });

        let key: String = "key".to_string();
        let get: Result<RedisElement, String> = redis.execute(Command::Get { key });

        assert!(get.is_err());
    }

    #[test]
    fn test_getset_fails_if_is_not_string() {
        let mut redis: Redis = Redis::new();

        let key: String = "key".to_string();
        let value = vec!["value".to_string(), "value2".to_string()];
        let _lpush = redis.execute(Command::Lpush { key, value });

        let key: String = "key".to_string();
        let get: Result<RedisElement, String> = redis.execute(Command::Get { key });

        assert!(get.is_err());
    }

    #[test]
    fn test_ping_returns_pong() {
        let mut redis: Redis = Redis::new();

        let ping: Result<RedisElement, String> = redis.execute(Command::Ping);

        assert_eq!("PONG".to_string(), ping.unwrap().to_string());
    }

    #[test]
    fn test_incrby_with_2_as_value() {
        let mut redis: Redis = Redis::new();

        let key: String = "key".to_string();
        let value: String = "1".to_string();
        let _set = redis.execute(Command::Set { key, value });

        let key: String = "key".to_string();
        let increment: u32 = 1;
        let _incrby = redis.execute(Command::Incrby { key, increment });

        let key: String = "key".to_string();
        let get: Result<RedisElement, String> = redis.execute(Command::Get { key });

        let key: String = "key".to_string();
        let increment: u32 = 2;
        let _incrby = redis.execute(Command::Incrby { key, increment });

        let key: String = "key".to_string();
        let second_get: Result<RedisElement, String> = redis.execute(Command::Get { key });

        assert_eq!("2".to_string(), get.unwrap().to_string());
        assert_eq!("4".to_string(), second_get.unwrap().to_string());
    }

    #[test]
    fn test_incrby_value_err_initial_value_string() {
        let mut redis: Redis = Redis::new();

        let key: String = "key".to_string();
        let value: String = "hola".to_string();
        let _set = redis.execute(Command::Set { key, value });

        let key: String = "key".to_string();
        let increment: u32 = 1;
        let incrby = redis.execute(Command::Incrby { key, increment });

        assert!(incrby.is_err());
    }

    #[test]
    fn test_incrby_not_saved_value() {
        let mut redis: Redis = Redis::new();

        let key: String = "key".to_string();
        let increment: u32 = 1;
        let _incrby = redis.execute(Command::Incrby { key, increment });

        let key: String = "key".to_string();
        let get: Result<RedisElement, String> = redis.execute(Command::Get { key });

        let key: String = "key".to_string();
        let second_get: Result<RedisElement, String> = redis.execute(Command::Get { key });

        assert_eq!("1".to_string(), get.unwrap().to_string());
        assert_ne!("10".to_string(), second_get.unwrap().to_string());
    }

    #[test]
    fn test_decrby_on_new_key() {
        let mut redis: Redis = Redis::new();

        let key: String = "key".to_string();
        let decrement: u32 = 3;
        let _decrby = redis.execute(Command::Decrby { key, decrement });

        let key: String = "key".to_string();
        let get: Result<RedisElement, String> = redis.execute(Command::Get { key });

        assert_eq!("-3".to_string(), get.unwrap().to_string());
    }

    #[test]
    fn test_decrby_on_existing_key() {
        let mut redis: Redis = Redis::new();

        let key: String = "key".to_string();
        let value: String = "5".to_string();
        let _set = redis.execute(Command::Set { key, value });

        let key: String = "key".to_string();
        let decrement: u32 = 3;
        let _decrby = redis.execute(Command::Decrby { key, decrement });

        let key: String = "key".to_string();
        let get: Result<RedisElement, String> = redis.execute(Command::Get { key });

        assert_eq!("2".to_string(), get.unwrap().to_string());
    }

    #[test]
    fn test_mset_sets_2_values() {
        let mut redis: Redis = Redis::new();

        let key_values = vec![
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string()),
        ];
        let _mset = redis.execute(Command::Mset { key_values });

        let key = "key1".to_string();
        let get: Result<RedisElement, String> = redis.execute(Command::Get { key });
        assert_eq!("value1".to_string(), get.unwrap().to_string());

        let key = "key2".to_string();
        let get: Result<RedisElement, String> = redis.execute(Command::Get { key });
        assert_eq!("value2".to_string(), get.unwrap().to_string());
    }

    #[test]
    fn test_set_element_and_getdel() {
        let mut redis: Redis = Redis::new();

        let value: String = "value".to_string();
        let key: String = "key".to_string();
        let _set = redis.execute(Command::Set { key, value });

        let key: String = "key".to_string();
        let get: Result<RedisElement, String> = redis.execute(Command::Get { key });

        let key: String = "key".to_string();
        let getdel: Result<RedisElement, String> = redis.execute(Command::Getdel { key });

        assert_eq!("value".to_string(), get.unwrap().to_string());
        assert_eq!("value".to_string(), getdel.unwrap().to_string());

        let key: String = "key".to_string();
        let get: Result<RedisElement, String> = redis.execute(Command::Get { key });
        assert_eq!("(nil)", get.unwrap().to_string());
    }

    #[test]
    fn test_getdel_without_previews_saving_err() {
        let mut redis: Redis = Redis::new();

        let key: String = "key".to_string();
        let getdel: Result<RedisElement, String> = redis.execute(Command::Getdel { key });
        assert_eq!("(nil)", getdel.unwrap().to_string());
    }

    #[test]
    fn test_dbsize() {
        let mut redis: Redis = Redis::new();

        let dbsize: Result<RedisElement, String> = redis.execute(Command::Dbsize);
        assert_eq!("0".to_string(), dbsize.unwrap().to_string());

        let value: String = "value".to_string();
        let key: String = "key".to_string();
        let _set = redis.execute(Command::Set { key, value });

        let dbsize: Result<RedisElement, String> = redis.execute(Command::Dbsize);
        assert_eq!("1".to_string(), dbsize.unwrap().to_string());

        let key: String = "key".to_string();
        let _getdel: Result<RedisElement, String> = redis.execute(Command::Getdel { key });

        let dbsize: Result<RedisElement, String> = redis.execute(Command::Dbsize);
        assert_eq!("0".to_string(), dbsize.unwrap().to_string());
    }

    #[test]
    fn test_set_element_and_del() {
        let mut redis: Redis = Redis::new();

        let value: String = "value".to_string();
        let key: String = "key".to_string();
        let _set = redis.execute(Command::Set { key, value });

        let keys = vec!["key".to_string()];
        let del: Result<RedisElement, String> = redis.execute(Command::Del { keys });
        assert_eq!("1".to_string(), del.unwrap().to_string());

        let key: String = "key".to_string();
        let get: Result<RedisElement, String> = redis.execute(Command::Get { key });
        assert_eq!("(nil)", get.unwrap().to_string());
    }

    #[test]
    fn test_set_two_elements_and_del_both() {
        let mut redis: Redis = Redis::new();

        let value: String = "value".to_string();
        let key: String = "key1".to_string();
        let _set = redis.execute(Command::Set { key, value });

        let value: String = "value".to_string();
        let key: String = "key2".to_string();
        let _set = redis.execute(Command::Set { key, value });

        let keys = vec!["key1".to_string(), "key2".to_string()];
        let del: Result<RedisElement, String> = redis.execute(Command::Del { keys });

        assert_eq!("2".to_string(), del.unwrap().to_string());
    }

    #[test]
    fn test_append_adds_word() {
        let mut redis: Redis = Redis::new();

        let key: String = "key".to_string();
        let value: String = "value".to_string();
        let _set = redis.execute(Command::Set { key, value });

        let key: String = "key".to_string();
        let value: String = " appended".to_string();
        let _append = redis.execute(Command::Append { key, value });

        let key: String = "key".to_string();
        let get: Result<RedisElement, String> = redis.execute(Command::Get { key });
        assert_eq!("value appended".to_string(), get.unwrap().to_string());
    }

    #[test]
    fn test_append_on_non_existent_key() {
        let mut redis: Redis = Redis::new();

        let key: String = "key".to_string();
        let value: String = " appended".to_string();
        let _append = redis.execute(Command::Append { key, value });

        let key: String = "key".to_string();
        let get: Result<RedisElement, String> = redis.execute(Command::Get { key });

        assert_eq!(" appended".to_string(), get.unwrap().to_string());
    }

    #[test]
    fn test_set_two_elements_and_check_exists_equal_2() {
        let mut redis: Redis = Redis::new();

        let key: String = "key1".to_string();
        let value: String = "value".to_string();
        let _set = redis.execute(Command::Set { key, value });

        let key: String = "key2".to_string();
        let value: String = "value".to_string();
        let _set = redis.execute(Command::Set { key, value });

        let keys = vec!["key1".to_string(), "key2".to_string()];
        let exists: Result<RedisElement, String> = redis.execute(Command::Exists { keys });
        assert_eq!("2".to_string(), exists.unwrap().to_string());

        let keys = vec!["key1".to_string(), "key2".to_string(), "key3".to_string()];
        let exists: Result<RedisElement, String> = redis.execute(Command::Exists { keys });
        assert_eq!("2".to_string(), exists.unwrap().to_string());
    }

    #[test]
    fn test_copy_on_existing_key_fails() {
        let mut redis: Redis = Redis::new();

        let key: String = "key1".to_string();
        let value: String = "value1".to_string();
        let _set = redis.execute(Command::Set { key, value });

        let key: String = "key2".to_string();
        let value: String = "value2".to_string();
        let _set = redis.execute(Command::Set { key, value });

        let key_origin: String = "key1".to_string();
        let key_destination: String = "key2".to_string();
        let copy = redis.execute(Command::Copy {
            key_destination,
            key_origin,
        });

        assert!(copy.is_err());
    }

    #[test]
    fn test_copy_ok() {
        let mut redis: Redis = Redis::new();

        let key: String = "key1".to_string();
        let value: String = "value1".to_string();
        let _set = redis.execute(Command::Set { key, value });

        let key_origin: String = "key1".to_string();
        let key_destination: String = "key2".to_string();
        let copy = redis.execute(Command::Copy {
            key_destination,
            key_origin,
        });

        let key: String = "key2".to_string();
        let get = redis.execute(Command::Get { key });
        assert_eq!("value1".to_string(), get.unwrap().to_string());
    }

    #[test]
    fn test_set_and_rename() {
        let mut redis: Redis = Redis::new();

        let key: String = "key1".to_string();
        let value: String = "value1".to_string();
        let _set = redis.execute(Command::Set { key, value });

        let key_origin: String = "key1".to_string();
        let key_destination: String = "key2".to_string();
        let rename = redis.execute(Command::Rename {
            key_origin,
            key_destination,
        });
        assert!(rename.is_ok());

        let key: String = "key1".to_string();
        let get = redis.execute(Command::Get { key });
        assert_eq!("(nil)", get.unwrap().to_string());

        let key: String = "key2".to_string();
        let get = redis.execute(Command::Get { key });
        assert!(get.is_ok());
        assert_eq!("value1".to_string(), get.unwrap().to_string());
    }

    #[test]
    fn test_lindex_with_key_used_err() {
        let mut redis: Redis = Redis::new();

        let key: String = "key".to_string();
        let value = "value".to_string();
        let _set = redis.execute(Command::Set { key, value });

        let key: String = "key".to_string();
        let index = 1;
        let lindex = redis.execute(Command::Lindex { key, index });

        assert!(lindex.is_err());
    }

    #[test]
    fn test_lindex_ok() {
        let mut redis: Redis = Redis::new();

        let key: String = "key".to_string();
        let value = vec!["value".to_string(), "value2".to_string()];
        let _lpush = redis.execute(Command::Lpush { key, value });

        let key: String = "key".to_string();
        let index = 0;
        let lindex = redis.execute(Command::Lindex { key, index });

        println!("{:?}", redis);

        assert!(lindex.is_ok());
        assert_eq!("value2".to_string(), lindex.unwrap().to_string())
    }

    #[test]
    fn test_lindex_negative_index_ok() {
        let mut redis: Redis = Redis::new();

        let key: String = "key".to_string();
        let value = vec!["value".to_string(), "value2".to_string()];
        let _lpush = redis.execute(Command::Lpush { key, value });

        let key: String = "key".to_string();
        let index = -1;
        let lindex = redis.execute(Command::Lindex { key, index });

        assert!(lindex.is_ok());
        assert_eq!("value".to_string(), lindex.unwrap().to_string())
    }

    #[test]
    fn test_lindex_negative_index_result_nil_ok() {
        let mut redis: Redis = Redis::new();

        let key: String = "key".to_string();
        let value = vec!["value".to_string(), "value2".to_string()];
        let _lpush = redis.execute(Command::Lpush { key, value });

        let key: String = "key".to_string();
        let index = -3;
        let lindex = redis.execute(Command::Lindex { key, index });

        assert!(lindex.is_ok());
        assert_eq!("(nil)", lindex.unwrap().to_string());
    }

    #[test]
    fn test_llen_key_saved_as_string_ok() {
        let mut redis: Redis = Redis::new();

        let key: String = "key".to_string();
        let value = "value".to_string();
        let _set = redis.execute(Command::Set { key, value });

        let key: String = "key".to_string();
        let llen = redis.execute(Command::Llen { key });

        assert!(llen.is_err());
    }

    #[test]
    fn test_llen_key_not_found_ok() {
        let mut redis: Redis = Redis::new();

        let key: String = "key".to_string();
        let llen = redis.execute(Command::Llen { key });

        assert!(llen.is_ok());
        assert_eq!("0".to_string(), llen.unwrap().to_string())
    }

    #[test]
    fn test_llen_key_used_twice_ok() {
        let mut redis: Redis = Redis::new();

        let key: String = "key".to_string();
        let value = vec!["value".to_string(), "value2".to_string()];
        let _lpush = redis.execute(Command::Lpush { key, value });

        let key: String = "key".to_string();
        let value = vec!["value".to_string(), "value2".to_string()];
        let _lpush = redis.execute(Command::Lpush { key, value });

        let key: String = "key".to_string();
        let llen = redis.execute(Command::Llen { key });

        assert_eq!("4".to_string(), llen.unwrap().to_string())
    }

    #[test]
    fn test_lpush_ok() {
        let mut redis: Redis = Redis::new();

        let key: String = "key".to_string();
        let value = vec!["value".to_string(), "value2".to_string()];
        let lpush = redis.execute(Command::Lpush { key, value });

        assert!(lpush.is_ok());
        assert_eq!("2".to_string(), lpush.unwrap().to_string())
    }

    #[test]
    fn test_lpush_with_key_used_err() {
        let mut redis: Redis = Redis::new();

        let key: String = "key".to_string();
        let value = "value".to_string();
        let _set = redis.execute(Command::Set { key, value });

        let key: String = "key".to_string();
        let value = vec!["value".to_string(), "value2".to_string()];
        let lpush = redis.execute(Command::Lpush { key, value });

        assert!(lpush.is_err());
    }

    #[test]
    fn test_lpush_key_used_ok() {
        let mut redis: Redis = Redis::new();

        let key: String = "key".to_string();
        let value = vec!["value".to_string(), "value2".to_string()];
        let lpush = redis.execute(Command::Lpush { key, value });

        assert!(lpush.is_ok());
        assert_eq!("2".to_string(), lpush.unwrap().to_string());

        let key: String = "key".to_string();
        let value = vec!["value".to_string(), "value2".to_string()];
        let lpush = redis.execute(Command::Lpush { key, value });

        assert!(lpush.is_ok());
        assert_eq!("4".to_string(), lpush.unwrap().to_string())
    }

    #[test]
    fn test_lpush_key_used_check_ok() {
        let mut redis: Redis = Redis::new();

        let key: String = "key".to_string();
        let value = vec!["1".to_string(), "2".to_string()];
        let _lpush = redis.execute(Command::Lpush { key, value });

        let key: String = "key".to_string();
        let value = vec!["3".to_string(), "4".to_string()];
        let _lpush = redis.execute(Command::Lpush { key, value });

        let key: String = "key".to_string();
        let index = -1;
        let lindex = redis.execute(Command::Lindex { key, index });
        assert!(lindex.is_ok());
        assert_eq!("1".to_string(), lindex.unwrap().to_string());
        let key: String = "key".to_string();
        let index = -2;
        let lindex = redis.execute(Command::Lindex { key, index });
        assert!(lindex.is_ok());
        assert_eq!("2".to_string(), lindex.unwrap().to_string());
        let key: String = "key".to_string();
        let index = -3;
        let lindex = redis.execute(Command::Lindex { key, index });
        assert!(lindex.is_ok());
        assert_eq!("3".to_string(), lindex.unwrap().to_string());
        let key: String = "key".to_string();
        let index = -4;
        let lindex = redis.execute(Command::Lindex { key, index });
        assert!(lindex.is_ok());
        assert_eq!("4".to_string(), lindex.unwrap().to_string());
    }
}
