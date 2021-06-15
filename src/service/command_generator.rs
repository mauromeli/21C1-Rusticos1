use crate::entities::command::Command;
use core::time::Duration;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::time::SystemTime;

#[allow(dead_code)]
pub fn generate(params: Vec<String>) -> Result<Command, String> {
    if params.is_empty() {
        return Err("Params can't be empty".to_string());
    }

    let command = params.first().unwrap();
    let params = Vec::from(params.get(1..).unwrap());
    match command.to_lowercase().as_str() {
        "ping" => generate_ping(params),
        "copy" => generate_copy(params),
        "get" => generate_get(params),
        "getset" => generate_getset(params),
        "set" => generate_set(params),
        "del" => generate_del(params),
        "exists" => generate_exists(params),
        "expire" => generate_expire(params),
        "expireat" => generate_expireat(params),
        "persist" => generate_persist(params),
        "rename" => generate_rename(params),
        "touch" => generate_touch(params),
        "type" => generate_type(params),
        "incrby" => generate_incrby(params),
        "decrby" => generate_decrby(params),
        "getdel" => generate_getdel(params),
        "append" => generate_append(params),
        "mget" => generate_mget(params),
        "mset" => generate_mset(params),
        "dbsize" => generate_dbsize(params),

        "lindex" => generate_lindex(params),
        "lpush" => generate_lpush(params),
        "llen" => generate_llen(params),
        "sadd" => generate_sadd(params),
        "scard" => generate_scard(params),
        "sismember" => generate_sismember(params),
        "smembers" => generate_smembers(params),
        "srem" => generate_srem(params),
        _ => Err("Command not valid".to_string()),
    }
}

fn generate_ping(params: Vec<String>) -> Result<Command, String> {
    if params.len() > 1 {
        return Err("ERR wrong number of arguments for 'ping' command".to_string());
    }

    Ok(Command::Ping)
}

fn generate_copy(params: Vec<String>) -> Result<Command, String> {
    if params.len() != 2 {
        return Err("ERR wrong number of arguments for 'copy' command".to_string());
    }

    let key_origin = params[0].clone();
    let key_destination = params[1].clone();
    Ok(Command::Copy {
        key_origin,
        key_destination,
    })
}

fn generate_get(params: Vec<String>) -> Result<Command, String> {
    if params.len() != 1 {
        return Err("ERR wrong number of arguments for 'get' command".to_string());
    }

    let key = params[0].clone();
    Ok(Command::Get { key })
}

fn generate_getset(params: Vec<String>) -> Result<Command, String> {
    if params.len() != 2 {
        return Err("ERR wrong number of arguments for 'getset' command".to_string());
    }

    let key = params[0].clone();
    let value = params[1].clone();
    Ok(Command::Getset { key, value })
}

fn generate_set(params: Vec<String>) -> Result<Command, String> {
    if params.len() != 2 {
        return Err("ERR syntax error".to_string());
    }

    let key = params[0].clone();
    let value = params[1].clone();
    Ok(Command::Set { key, value })
}

fn generate_incrby(params: Vec<String>) -> Result<Command, String> {
    if params.len() != 2 {
        return Err("ERR syntax error".to_string());
    }

    let key = params[0].clone();
    let increment: Result<u32, _> = params[1].to_string().parse();

    if increment.is_err() {
        return Err("ERR value is not an integer or out of range".to_string());
    }

    let increment = increment.unwrap();
    Ok(Command::Incrby { key, increment })
}

fn generate_decrby(params: Vec<String>) -> Result<Command, String> {
    if params.len() != 2 {
        return Err("ERR syntax error".to_string());
    }

    let key = params[0].clone();
    let decrement: Result<u32, _> = params[1].to_string().parse();

    if decrement.is_err() {
        return Err("ERR value is not an integer or out of range".to_string());
    }

    let decrement = decrement.unwrap();
    Ok(Command::Decrby { key, decrement })
}

fn generate_getdel(params: Vec<String>) -> Result<Command, String> {
    if params.len() != 1 {
        return Err("ERR wrong number of arguments for 'getdel' command".to_string());
    }

    let key = params[0].clone();
    Ok(Command::Getdel { key })
}

fn generate_del(params: Vec<String>) -> Result<Command, String> {
    if params.is_empty() {
        return Err("ERR wrong number of arguments for 'del' command".to_string());
    }

    Ok(Command::Del { keys: params })
}

fn generate_append(params: Vec<String>) -> Result<Command, String> {
    if params.len() != 2 {
        return Err("ERR wrong number of arguments for 'append' command".to_string());
    }

    let key = params[0].clone();
    let value = params[1].clone();
    Ok(Command::Append { key, value })
}

fn generate_exists(params: Vec<String>) -> Result<Command, String> {
    if params.is_empty() {
        return Err("ERR wrong number of arguments for 'exists' command".to_string());
    }

    Ok(Command::Exists { keys: params })
}

fn generate_expire(params: Vec<String>) -> Result<Command, String> {
    if params.len() != 2 {
        return Err("ERR wrong number of arguments for 'expire' command".to_string());
    }

    let key = params[0].clone();
    //TODO: deberian poder ser segundos negativos, corregir
    let seconds: Result<u32, _> = params[1].to_string().parse();

    if seconds.is_err() {
        return Err("ERR value is not an integer or out of range".to_string());
    }

    let ttl = Duration::from_secs(seconds.unwrap().into());

    Ok(Command::Expire { key, ttl })
}

fn generate_expireat(params: Vec<String>) -> Result<Command, String> {
    if params.len() != 2 {
        return Err("ERR wrong number of arguments for 'expireat' command".to_string());
    }

    let key = params[0].clone();
    let seconds: Result<u32, _> = params[1].to_string().parse();

    if seconds.is_err() {
        return Err("ERR value is not an integer or out of range".to_string());
    }

    let ttl = SystemTime::UNIX_EPOCH + Duration::from_secs(seconds.unwrap().into());

    Ok(Command::Expireat { key, ttl })
}

fn generate_persist(params: Vec<String>) -> Result<Command, String> {
    if params.len() != 1 {
        return Err("ERR wrong number of arguments for 'persist' command".to_string());
    }

    let key = params[0].clone();
    Ok(Command::Persist { key })
}

fn generate_rename(params: Vec<String>) -> Result<Command, String> {
    if params.len() != 2 {
        return Err("ERR wrong number of arguments for 'rename' command".to_string());
    }

    let key_origin = params[0].clone();
    let key_destination = params[1].clone();
    Ok(Command::Rename {
        key_origin,
        key_destination,
    })
}

fn generate_touch(params: Vec<String>) -> Result<Command, String> {
    if params.is_empty() {
        return Err("ERR wrong number of arguments for 'touch' command".to_string());
    }

    Ok(Command::Touch { keys: params })
}

fn generate_type(params: Vec<String>) -> Result<Command, String> {
    if params.len() != 1 {
        return Err("ERR wrong number of arguments for 'type' command".to_string());
    }

    let key = params[0].clone();
    Ok(Command::Type { key })
}

fn generate_mget(params: Vec<String>) -> Result<Command, String> {
    if params.is_empty() {
        return Err("ERR wrong number of arguments for 'mget' command".to_string());
    }

    Ok(Command::Mget { keys: params })
}

fn generate_mset(params: Vec<String>) -> Result<Command, String> {
    if params.is_empty() || params.len() % 2 != 0 {
        return Err("ERR wrong number of arguments for 'mset' command".to_string());
    }

    let mut key_values: Vec<(String, String)> = Vec::new();
    for pair in params.chunks(2) {
        let tuple = (pair[0].to_string(), pair[1].to_string());
        key_values.push(tuple);
    }
    Ok(Command::Mset { key_values })
}

fn generate_dbsize(params: Vec<String>) -> Result<Command, String> {
    if !params.is_empty() {
        return Err("ERR wrong number of arguments for 'dbsize' command".to_string());
    }

    Ok(Command::Dbsize)
}

fn generate_lindex(params: Vec<String>) -> Result<Command, String> {
    if params.len() != 2 {
        return Err("ERR wrong number of arguments for 'lindex' command".to_string());
    }

    let key = params[0].clone();
    let index: Result<i32, _> = params[1].to_string().parse();

    if index.is_err() {
        return Err("ERR value is not an integer or out of range".to_string());
    }

    let index = index.unwrap();
    Ok(Command::Lindex { key, index })
}

fn generate_llen(params: Vec<String>) -> Result<Command, String> {
    if params.is_empty() {
        return Err("ERR wrong number of arguments for 'llen' command".to_string());
    }

    let key = params[0].to_string();
    Ok(Command::Llen { key })
}

fn generate_lpush(params: Vec<String>) -> Result<Command, String> {
    if params.len() <= 1 {
        return Err("ERR wrong number of arguments for 'lpush' command".to_string());
    }

    let key = params[0].clone();
    let values = Vec::from(params.get(1..).unwrap());

    Ok(Command::Lpush { key, value: values })
}

fn generate_sadd(params: Vec<String>) -> Result<Command, String> {
    if params.len() <= 1 {
        return Err("ERR wrong number of arguments for 'sadd' command".to_string());
    }

    let key = params[0].clone();
    let vector = Vec::from(params.get(1..).unwrap());
    let values = HashSet::from_iter(vector);

    Ok(Command::Sadd { key, values })
}

fn generate_scard(params: Vec<String>) -> Result<Command, String> {
    if params.is_empty() {
        return Err("ERR wrong number of arguments for 'scard' command".to_string());
    }
    let key = params[0].clone();
    Ok(Command::Scard { key })
}

fn generate_sismember(params: Vec<String>) -> Result<Command, String> {
    if params.len() != 2 {
        return Err("ERR wrong number of arguments for 'sismember' command".to_string());
    }

    let key = params[0].clone();
    let value = params[1].clone();
    Ok(Command::Sismember { key, value })
}

fn generate_srem(params: Vec<String>) -> Result<Command, String> {
    if params.len() <= 1 {
        return Err("ERR wrong number of arguments for 'srem' command".to_string());
    }

    let key = params[0].clone();
    let vector = Vec::from(params.get(1..).unwrap());
    let values = HashSet::from_iter(vector);

    Ok(Command::Srem { key, values })
}

fn generate_smembers(params: Vec<String>) -> Result<Command, String> {
    if params.is_empty() {
        return Err("ERR wrong number of arguments for 'smembers' command".to_string());
    }
    let key = params[0].clone();
    Ok(Command::Smembers { key })
}

#[allow(unused_imports)]
mod test {
    use crate::entities::command::Command;
    use crate::service::command_generator::generate;
    use core::time::Duration;
    use std::collections::HashSet;
    use std::time::SystemTime;

    #[test]
    fn generate_command_with_params_empty_err() {
        let params = vec![];
        let result = generate(params);

        assert!(result.is_err())
    }

    #[test]
    fn generate_command_with_command_invalid_err() {
        let params = vec!["metodo".to_string()];
        let result = generate(params);

        assert!(result.is_err())
    }

    #[test]
    fn generate_command_with_command_ping() {
        let params = vec!["ping".to_string()];
        let result = generate(params);

        assert!(result.is_ok());
        assert!(match result.unwrap() {
            Command::Ping => true,
            _ => false,
        });
    }

    #[test]
    fn generate_command_copy_without_params_err() {
        let params = vec!["copy".to_string()];
        let result = generate(params);

        assert!(result.is_err())
    }

    #[test]
    fn generate_command_copy_with_one_param_err() {
        let params = vec!["copy".to_string(), "key".to_string()];
        let result = generate(params);

        assert!(result.is_err())
    }

    #[test]
    fn generate_command_copy_ok() {
        let params = vec!["copy".to_string(), "key".to_string(), "key1".to_string()];
        let result = generate(params);

        let _key = "key".to_string();
        let _key2 = "key1".to_string();
        assert!(result.is_ok());
        assert!(match result.unwrap() {
            Command::Copy {
                key_origin: _key,
                key_destination: _key2,
            } => true,
            _ => false,
        });
    }

    #[test]
    fn generate_command_get_without_param_err() {
        let params = vec!["get".to_string()];
        let result = generate(params);

        assert!(result.is_err())
    }

    #[test]
    fn generate_command_get_ok() {
        let params = vec!["get".to_string(), "key".to_string()];
        let result = generate(params);

        let _key = "key".to_string();
        assert!(result.is_ok());
        assert!(match result.unwrap() {
            Command::Get { key: _key } => true,
            _ => false,
        });
    }

    #[test]
    fn generate_command_getset_without_param_err() {
        let params = vec!["getset".to_string()];
        let result = generate(params);

        assert!(result.is_err())
    }

    #[test]
    fn generate_command_getset_with_one_param_err() {
        let params = vec!["getset".to_string(), "key".to_string()];
        let result = generate(params);

        assert!(result.is_err())
    }

    #[test]
    fn generate_command_getset_ok() {
        let params = vec!["getset".to_string(), "key".to_string(), "value".to_string()];
        let result = generate(params);

        let _key = "key".to_string();
        let _value = "value".to_string();
        assert!(result.is_ok());
        assert!(match result.unwrap() {
            Command::Getset {
                key: _key,
                value: _value,
            } => true,
            _ => false,
        });
    }

    #[test]
    fn generate_command_set_without_param_err() {
        let params = vec!["set".to_string()];
        let result = generate(params);

        assert!(result.is_err())
    }

    #[test]
    fn generate_command_set_with_one_param_err() {
        let params = vec!["set".to_string(), "key".to_string()];
        let result = generate(params);

        assert!(result.is_err())
    }

    #[test]
    fn generate_command_set_ok() {
        let params = vec!["set".to_string(), "key".to_string(), "value".to_string()];
        let result = generate(params);

        let _key = "key".to_string();
        let _value = "value".to_string();
        assert!(result.is_ok());
        assert!(match result.unwrap() {
            Command::Set {
                key: _key,
                value: _value,
            } => true,
            _ => false,
        });
    }

    #[test]
    fn generate_command_del_without_param_err() {
        let params = vec!["del".to_string()];
        let result = generate(params);

        assert!(result.is_err())
    }

    #[test]
    fn generate_command_del_ok() {
        let params = vec!["del".to_string(), "key".to_string()];
        let result = generate(params);

        let _keys = vec!["key".to_string()];
        assert!(result.is_ok());
        assert!(match result.unwrap() {
            Command::Del { keys: _keys } => true,
            _ => false,
        });
    }

    #[test]
    fn generate_command_mget_without_param_err() {
        let params = vec!["mget".to_string()];
        let result = generate(params);

        assert!(result.is_err())
    }

    #[test]
    fn generate_command_mget_ok() {
        let params = vec!["mget".to_string(), "key1".to_string(), "key2".to_string()];
        let result = generate(params);

        let _keys = vec!["key1".to_string(), "key2".to_string()];
        assert!(result.is_ok());
        assert!(match result.unwrap() {
            Command::Mget { keys: _keys } => true,
            _ => false,
        });
    }

    #[test]
    fn generate_command_mset_without_param_err() {
        let params = vec!["mset".to_string()];
        let result = generate(params);

        assert!(result.is_err())
    }

    #[test]
    fn generate_command_mset_with_missing_value_err() {
        let params = vec![
            "mset".to_string(),
            "key1".to_string(),
            "value1".to_string(),
            "key2".to_string(),
        ];
        let result = generate(params);

        assert!(result.is_err())
    }

    #[test]
    fn generate_command_mset_ok() {
        let params = vec![
            "mset".to_string(),
            "key1".to_string(),
            "value1".to_string(),
            "key2".to_string(),
            "value2".to_string(),
        ];
        let result = generate(params);

        let _pairs = vec![
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string()),
        ];
        assert!(result.is_ok());
        assert!(match result.unwrap() {
            Command::Mset { key_values: _pairs } => true,
            _ => false,
        });
    }

    #[test]
    fn generate_command_exists_without_param_err() {
        let params = vec!["exists".to_string()];
        let result = generate(params);

        assert!(result.is_err())
    }

    #[test]
    fn generate_command_exists_ok() {
        let params = vec!["exists".to_string(), "key".to_string()];
        let result = generate(params);

        let _keys = vec!["key".to_string()];
        assert!(result.is_ok());

        assert!(match result.unwrap() {
            Command::Exists { keys: _keys } => true,
            _ => false,
        });

        let params = vec!["exists".to_string(), "key".to_string()];
        let result = generate(params);

        assert!(match result.unwrap() {
            Command::Ping => false,
            _ => true,
        });
    }

    #[test]
    fn generate_command_rename_without_param_err() {
        let params = vec!["rename".to_string()];
        let result = generate(params);

        assert!(result.is_err())
    }

    #[test]
    fn generate_command_rename_ok() {
        let params = vec!["rename".to_string(), "key1".to_string(), "key2".to_string()];
        let result = generate(params);

        let _key_origin = "key1".to_string();
        let _key_destination = "key2".to_string();

        assert!(result.is_ok());

        assert!(match result.unwrap() {
            Command::Rename {
                key_origin: _key_origin,
                key_destination: _key_destination,
            } => true,
            _ => false,
        });
    }

    #[test]
    fn generate_command_expire_without_param_err() {
        let params = vec!["expire".to_string()];
        let result = generate(params);

        assert!(result.is_err());
    }

    #[test]
    fn generate_command_expire_with_fractional_time_err() {
        let params = vec!["expire".to_string(), "key".to_string(), "10.5".to_string()];
        let result = generate(params);

        assert!(result.is_err());
    }

    #[test]
    fn generate_command_expire_ok() {
        let params = vec!["expire".to_string(), "key".to_string(), "1".to_string()];
        let result = generate(params);

        let _key = "key".to_string();
        let _ttl = Duration::from_secs(1);

        assert!(result.is_ok());

        assert!(match result.unwrap() {
            Command::Expire {
                key: _key,
                ttl: _ttl,
            } => true,
            _ => false,
        });
    }

    #[test]
    fn generate_command_expireat_without_param_err() {
        let params = vec!["expireat".to_string()];
        let result = generate(params);

        assert!(result.is_err());
    }

    #[test]
    fn generate_command_expireat_with_fractional_time_err() {
        let params = vec![
            "expireat".to_string(),
            "key".to_string(),
            "10.5".to_string(),
        ];
        let result = generate(params);

        assert!(result.is_err());
    }

    #[test]
    fn generate_command_expireat_ok() {
        let params = vec!["expireat".to_string(), "key".to_string(), "1".to_string()];
        let result = generate(params);

        let _key = "key".to_string();
        let _ttl = SystemTime::UNIX_EPOCH + Duration::from_secs(1);

        assert!(result.is_ok());

        assert!(match result.unwrap() {
            Command::Expireat {
                key: _key,
                ttl: _ttl,
            } => true,
            _ => false,
        });
    }

    #[test]
    fn generate_command_persist_without_param_err() {
        let params = vec!["persist".to_string()];
        let result = generate(params);

        assert!(result.is_err())
    }

    #[test]
    fn generate_command_persist_ok() {
        let params = vec!["persist".to_string(), "key".to_string()];
        let result = generate(params);

        let _key = "key".to_string();
        assert!(result.is_ok());

        assert!(match result.unwrap() {
            Command::Persist { key: _key } => true,
            _ => false,
        });
    }

    #[test]
    fn generate_command_touch_without_param_err() {
        let params = vec!["touch".to_string()];
        let result = generate(params);

        assert!(result.is_err())
    }

    #[test]
    fn generate_command_touch_ok() {
        let params = vec!["touch".to_string(), "key1".to_string(), "key2".to_string()];
        let result = generate(params);

        let _keys = vec!["key1".to_string(), "key2".to_string()];
        assert!(result.is_ok());

        assert!(match result.unwrap() {
            Command::Touch { keys: _keys } => true,
            _ => false,
        });
    }

    #[test]
    fn generate_command_type_without_param_err() {
        let params = vec!["type".to_string()];
        let result = generate(params);

        assert!(result.is_err())
    }

    #[test]
    fn generate_command_type_ok() {
        let params = vec!["type".to_string(), "key".to_string()];
        let result = generate(params);

        let _key = "key".to_string();

        assert!(result.is_ok());

        assert!(match result.unwrap() {
            Command::Type { key: _key } => true,
            _ => false,
        });
    }

    #[test]
    fn generate_command_incrby_without_param_err() {
        let params = vec!["incrby".to_string()];
        let result = generate(params);

        assert!(result.is_err());

        let params = vec!["incrby".to_string(), "key".to_string(), "hola".to_string()];
        let result = generate(params);

        assert!(result.is_err())
    }

    #[test]
    fn generate_command_incrby_ok() {
        let params = vec!["incrby".to_string(), "key1".to_string(), "1".to_string()];
        let result = generate(params);

        let _key = "key1".to_string();

        assert!(result.is_ok());

        assert!(match result.unwrap() {
            Command::Incrby {
                key: _key,
                increment: 1,
            } => true,
            _ => false,
        });
    }

    #[test]
    fn generate_command_decrby_without_param_err() {
        let params = vec!["decrby".to_string()];
        let result = generate(params);

        assert!(result.is_err());

        let params = vec!["decrby".to_string(), "key".to_string(), "hola".to_string()];
        let result = generate(params);

        assert!(result.is_err())
    }

    #[test]
    fn generate_command_decrby_ok() {
        let params = vec!["decrby".to_string(), "key1".to_string(), "1".to_string()];
        let result = generate(params);

        let _key = "key1".to_string();

        assert!(result.is_ok());

        assert!(match result.unwrap() {
            Command::Decrby {
                key: _key,
                decrement: 1,
            } => true,
            _ => false,
        });
    }

    #[test]
    fn generate_command_getdel_without_param_err() {
        let params = vec!["getdel".to_string()];
        let result = generate(params);

        assert!(result.is_err())
    }

    #[test]
    fn generate_command_getdel_ok() {
        let params = vec!["getdel".to_string(), "key".to_string()];
        let result = generate(params);

        let _key = "key".to_string();
        assert!(result.is_ok());
        assert!(match result.unwrap() {
            Command::Getdel { key: _key } => true,
            _ => false,
        });

        let params = vec!["getdel".to_string(), "key".to_string()];
        let result = generate(params);

        assert!(match result.unwrap() {
            Command::Ping => false,
            _ => true,
        });
    }

    #[test]
    fn generate_command_append_without_param_err() {
        let params = vec!["append".to_string()];
        let result = generate(params);

        assert!(result.is_err())
    }

    #[test]
    fn generate_command_append_ok() {
        let params = vec!["append".to_string(), "key".to_string(), "Value".to_string()];
        let result = generate(params);

        let _key = "key".to_string();
        let _value = "Value".to_string();

        assert!(result.is_ok());
        assert!(match result.unwrap() {
            Command::Append {
                key: _key,
                value: _value,
            } => true,
            _ => false,
        });

        let params = vec!["append".to_string(), "key".to_string(), "Value".to_string()];
        let result = generate(params);

        assert!(match result.unwrap() {
            Command::Ping => false,
            _ => true,
        });
    }

    #[test]
    fn generate_command_with_command_dbsize() {
        let params = vec!["dbsize".to_string()];
        let result = generate(params);

        assert!(result.is_ok());
        assert!(match result.unwrap() {
            Command::Dbsize => true,
            _ => false,
        });
    }

    #[test]
    fn generate_command_lindex_incorrect_params_err() {
        let params = vec!["lindex".to_string()];
        let result = generate(params);

        assert!(result.is_err());

        let params = vec!["lindex".to_string(), "key".to_string()];
        let result = generate(params);

        assert!(result.is_err());

        let params = vec!["lindex".to_string(), "key".to_string(), "key".to_string()];
        let result = generate(params);

        assert!(result.is_err());

        let params = vec![
            "lindex".to_string(),
            "key".to_string(),
            "1".to_string(),
            "value".to_string(),
        ];
        let result = generate(params);

        assert!(result.is_err());
    }

    #[test]
    fn generate_command_lindex_ok() {
        let params = vec!["lindex".to_string(), "key".to_string(), "1".to_string()];
        let result = generate(params);

        let _key = "key".to_string();
        let _index = 1;
        assert!(result.is_ok());
        assert!(match result.unwrap() {
            Command::Lindex {
                key: _key,
                index: _index,
            } => true,
            _ => false,
        });

        let params = vec!["lindex".to_string(), "key".to_string(), "-1".to_string()];
        let result = generate(params);

        let _key = "key".to_string();
        let _index = -1;
        assert!(result.is_ok());
        assert!(match result.unwrap() {
            Command::Lindex {
                key: _key,
                index: _index,
            } => true,
            _ => false,
        });
    }

    #[test]
    fn generate_command_llen_without_param_err() {
        let params = vec!["llen".to_string()];
        let result = generate(params);

        assert!(result.is_err())
    }

    #[test]
    fn generate_command_llen_ok() {
        let params = vec!["llen".to_string(), "key".to_string()];
        let result = generate(params);

        let _key = "key".to_string();
        assert!(result.is_ok());
        assert!(match result.unwrap() {
            Command::Llen { key: _key } => true,
            _ => false,
        });
    }

    #[test]
    fn generate_command_lpush_incorrect_params_err() {
        let params = vec!["lpush".to_string()];
        let result = generate(params);

        assert!(result.is_err());

        let params = vec!["lpush".to_string(), "key".to_string()];
        let result = generate(params);

        assert!(result.is_err())
    }

    #[test]
    fn generate_command_lpush_ok() {
        let params = vec!["lpush".to_string(), "key".to_string(), "value".to_string()];
        let result = generate(params);

        let _key = "key".to_string();
        let _value = vec!["value".to_string()];
        assert!(result.is_ok());
        assert!(match result.unwrap() {
            Command::Lpush {
                key: _key,
                value: _value,
            } => true,
            _ => false,
        });
    }

    #[test]
    fn generate_command_sadd_incorrect_params_err() {
        let params = vec!["sadd".to_string()];
        let result = generate(params);

        assert!(result.is_err());

        let params = vec!["sadd".to_string(), "key".to_string()];
        let result = generate(params);

        assert!(result.is_err())
    }

    #[test]
    fn generate_command_sadd_ok() {
        let params = vec!["sadd".to_string(), "key".to_string(), "value".to_string()];
        let result = generate(params);

        let _key = "key".to_string();
        let mut _values = HashSet::new();
        _values.insert("value1".to_string());
        _values.insert("value2".to_string());
        assert!(result.is_ok());
        assert!(match result.unwrap() {
            Command::Sadd {
                key: _key,
                values: _values,
            } => true,
            _ => false,
        });
    }

    #[test]
    fn generate_command_scard_without_param_err() {
        let params = vec!["scard".to_string()];
        let result = generate(params);

        assert!(result.is_err())
    }

    #[test]
    fn generate_command_scard_ok() {
        let params = vec!["scard".to_string(), "key".to_string()];
        let result = generate(params);

        let _key = "key".to_string();
        assert!(result.is_ok());
        assert!(match result.unwrap() {
            Command::Scard { key: _key } => true,
            _ => false,
        });
    }

    #[test]
    fn generate_command_sismember_without_param_err() {
        let params = vec!["sismember".to_string()];
        let result = generate(params);

        assert!(result.is_err())
    }

    #[test]
    fn generate_command_sismember_ok() {
        let params = vec![
            "sismember".to_string(),
            "key".to_string(),
            "value".to_string(),
        ];
        let result = generate(params);

        let _key = "key".to_string();
        let _value = "value".to_string();

        assert!(result.is_ok());
        assert!(match result.unwrap() {
            Command::Sismember {
                key: _key,
                value: _value,
            } => true,
            _ => false,
        });
    }

    #[test]
    fn generate_command_srem_incorrect_params_err() {
        let params = vec!["srem".to_string()];
        let result = generate(params);

        assert!(result.is_err());

        let params = vec!["srem".to_string(), "key".to_string()];
        let result = generate(params);

        assert!(result.is_err())
    }

    #[test]
    fn generate_command_srem_ok() {
        let params = vec!["srem".to_string(), "key".to_string(), "value".to_string()];
        let result = generate(params);

        let _key = "key".to_string();
        let mut _values = HashSet::new();
        _values.insert("value1".to_string());
        _values.insert("value2".to_string());
        assert!(result.is_ok());
        assert!(match result.unwrap() {
            Command::Srem {
                key: _key,
                values: _values,
            } => true,
            _ => false,
        });
    }

    #[test]
    fn generate_command_smembers_without_param_err() {
        let params = vec!["smembers".to_string()];
        let result = generate(params);

        assert!(result.is_err())
    }

    #[test]
    fn generate_command_smembers_ok() {
        let params = vec!["smembers".to_string(), "key".to_string()];
        let result = generate(params);

        let _key = "key".to_string();
        assert!(result.is_ok());
        assert!(match result.unwrap() {
            Command::Smembers { key: _key } => true,
            _ => false,
        });
    }
}
