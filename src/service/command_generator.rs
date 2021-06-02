use crate::entities::command::Command;

#[allow(dead_code)]
pub fn generate(params: Vec<String>) -> Result<Command, String> {
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
        return Err("ERR wrong number of arguments for 'copy' command".to_string());
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
        return Err("ERR wrong number of arguments for 'exists' command".to_string());
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

mod test {

}