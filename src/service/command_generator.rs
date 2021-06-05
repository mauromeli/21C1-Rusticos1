use crate::entities::command::Command;

#[allow(dead_code)]
pub fn generate(params: Vec<String>) -> Result<Command, String> {
    if params.is_empty() {
        return Err("Params can't be empty".to_string());
    }

    let command = params.first().unwrap();
    let params = params.get(1..).unwrap();
    match command.to_lowercase().as_str() {
        "ping" => generate_ping(Vec::from(params)),
        "copy" => generate_copy(Vec::from(params)),
        "get" => generate_get(Vec::from(params)),
        "set" => generate_set(Vec::from(params)),
        "del" => generate_del(Vec::from(params)),
        "exists" => generate_exists(Vec::from(params)),
        "rename" => generate_rename(Vec::from(params)),
        "incrby" => generate_incrby(Vec::from(params)),
        "getdel" => generate_getdel(Vec::from(params)),
        "append" => generate_append(Vec::from(params)),
        "dbsize" => generate_dbsize(Vec::from(params)),
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

fn generate_dbsize(params: Vec<String>) -> Result<Command, String> {
    if !params.is_empty() {
        return Err("ERR wrong number of arguments for 'dbsize' command".to_string());
    }

    Ok(Command::Dbsize)
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

        let params = vec!["exists".to_string(), "key".to_string(), "key2".to_string()];
        let result = generate(params);

        assert!(match result.unwrap() {
            Command::Ping => false,
            _ => true,
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

        let params = vec!["exists".to_string(), "key".to_string(), "2".to_string()];
        let result = generate(params);

        assert!(match result.unwrap() {
            Command::Ping => false,
            _ => true,
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
}
