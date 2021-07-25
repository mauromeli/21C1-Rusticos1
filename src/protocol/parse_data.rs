use crate::entities::redis_element::RedisElement;
use crate::protocol::encode::encode;
use std::iter::FromIterator;
use crate::protocol::type_data::TypeData;

/// Parsea el comando recibido (`TypeData`) a un `Vec<String>`.
///
/// # Arguments
///
/// * `data` - Comando, representado como `TypeData`.
pub fn parse_command(data: TypeData) -> Vec<String> {
    let empty_vector = Vec::new();
    if let Ok(vector) = parse_array(data) {
        return vector;
    }
    empty_vector
}

/// Parsea la respuesta de un comando, en caso de éxito, a bytes (`Vec<u8>`).
///
/// # Arguments
///
/// * `redis_element` - Respuesta de un comando, representado como `RedisElement`.
pub fn parse_response_ok(redis_element: RedisElement) -> Vec<u8> {
    encode(parse_response(redis_element))
}

/// Parsea la respuesta de un comando, en caso de error, a bytes (`Vec<u8>`).
///
/// # Arguments
///
/// * `error` - Respuesta de error un comando, representado como `String`.
pub fn parse_response_error(error: String) -> Vec<u8> {
    encode(TypeData::Error(error))
}

/// Parsea un `RedisElement` a un `TypeData`.
///
/// # Arguments
///
/// * `redis_element` - Redis element.
fn parse_response(redis_element: RedisElement) -> TypeData {
    match redis_element {
        RedisElement::String(string) => {
            let number = string.parse::<i64>();
            match number {
                Ok(number) => TypeData::Integer(number),
                Err(_) => TypeData::BulkString(string),
            }
        }
        RedisElement::List(list) => parse_list_and_set(list),
        RedisElement::Set(set) => parse_list_and_set(Vec::from_iter(set)),
        RedisElement::Nil => TypeData::Nil,
        RedisElement::SimpleString(string) => TypeData::String(string),
    }
}

/// Parsea un `Vec<String>` a un `TypeData::Array`.
///
/// # Arguments
///
/// * `vector_re` - Vector a parsear.
fn parse_list_and_set(vector_re: Vec<String>) -> TypeData {
    let mut vector = Vec::new();
    for element in vector_re {
        let type_data = parse_response(RedisElement::String(element));
        vector.push(type_data);
    }
    TypeData::Array(vector)
}


/// Intenta parsear un `TypeData` a un `Vec<String>`.
///
/// En caso de que falle, devuelve un error de tipo `String`. Sino, devuelve un `Vec<String>`.
///
/// # Arguments
///
/// * `type_data` - Type data.
fn parse_array(type_data: TypeData) -> Result<Vec<String>, String> {
    match type_data {
        TypeData::Array(vec) => {
            let mut vector = Vec::new();
            for element in vec {
                if let Ok(result) = parse_type_data(element.clone()) {
                    vector.push(result);
                }
            }
            Ok(vector)
        }
        _ => Err("Error comando ingresado".to_string()),
    }
}

/// Intenta parsear un `TypeData` a un `String`.
///
/// En caso de el `TypeData` no exista, devuelve un error de tipo `String`.
///
/// # Arguments
///
/// * `type_data` - Type data.
fn parse_type_data(type_data: TypeData) -> Result<String, String> {
    match type_data {
        TypeData::String(string) => Ok(string),
        TypeData::Integer(integer) => Ok(integer.to_string()),
        TypeData::BulkString(bulkstring) => Ok(bulkstring),
        _ => Err("Error tipo de dato".to_string()),
    }
}
