use crate::entities::command::Command;

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
        "rename" => generate_rename(params),
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

#[allow(unused_imports)]
mod test {
    use crate::entities::command::Command;
    use crate::service::command_generator::generate;

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
}
