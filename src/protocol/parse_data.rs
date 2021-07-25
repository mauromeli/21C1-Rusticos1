use crate::entities::redis_element::RedisElement;
use crate::protocol::decode::TypeData;
use crate::protocol::encode::encode;
use std::iter::FromIterator;

pub fn parse_command(data: TypeData) -> Vec<String> {
    let empty_vector = Vec::new();
    if let Ok(vector) = parse_array(data) {
        return vector;
    }
    empty_vector
}

pub fn parse_response_ok(redis_element: RedisElement) -> Vec<u8> {
    encode(parse_response(redis_element))
}

pub fn parse_response_error(error: String) -> Vec<u8> {
    encode(TypeData::Error(error))
}

fn parse_response(redis_element: RedisElement) -> TypeData {
    match redis_element {
        RedisElement::String(string) => {
            if string == "OK" || string == "PONG" {
                return TypeData::String(string);
            }
            let number = string.parse::<i64>();
            match number {
                Ok(number) => TypeData::Integer(number),
                Err(_) => TypeData::BulkString(string),
            }
        }
        RedisElement::List(list) => parse_list_and_set(list),
        RedisElement::Set(set) => parse_list_and_set(Vec::from_iter(set)),
        RedisElement::Nil => TypeData::Nil,
        RedisElement::SpecialString(string) => {
            return TypeData::String(string);
        }
    }
}

fn parse_list_and_set(vector_re: Vec<String>) -> TypeData {
    let mut vector = Vec::new();
    for element in vector_re {
        let type_data = parse_response(RedisElement::String(element));
        vector.push(type_data);
    }
    TypeData::Array(vector)
}

fn parse_array(type_data: TypeData) -> Result<Vec<String>, String> {
    match type_data {
        TypeData::Array(vec) => {
            let mut vector = Vec::new();
            for element in vec {
                let result = parse_type_data(element).unwrap();
                vector.push(result);
            }
            Ok(vector)
        }
        _ => Err("Error comando ingresado".to_string()),
    }
}

fn parse_type_data(type_data: TypeData) -> Result<String, String> {
    match type_data {
        TypeData::String(string) => Ok(string),
        TypeData::Integer(integer) => Ok(integer.to_string()),
        TypeData::BulkString(bulkstring) => Ok(bulkstring),
        _ => Err("Error tipo de dato".to_string()),
    }
}
