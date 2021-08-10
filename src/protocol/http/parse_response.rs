use crate::entities::redis_element::RedisElement;
use std::iter::FromIterator;
/// Inicio del formato que se debe devolver como respuesta para Integer.
const INTEGER: &str = "(integer) ";
/// Inicio del formato que se debe devolver como respuesta para String.
const STRING: &str = "\"";
/// Inicio del formato que se debe devolver como respuesta para List y Set.
const START_LIST: &str = ") \"";
/// Final del formato que se debe devolver como respuesta para List y Set.
const END_LIST: &str = "\" <br>";
/// Formato que se debe devolver como respuesta para Nil.
const NIL: &str = "(nil)";
/// Formato que se debe devolver como respuesta para List y Set vacíos.
const EMPTY_LIST_SET: &str = "(empty list or set)";

/// Parsea la respuesta que debe mostrarse en el html.
///
/// Retorna un `String` que representa la respuesta a mostrar.
///
/// # Arguments
///
/// * `redis_element` - Respuesta del comando
pub fn parse_response_rest(redis_element: RedisElement) -> String {
    match redis_element {
        RedisElement::String(string) => {
            if let Ok(number) = convert_to_number(string.clone()) {
                [INTEGER.to_string(), number.to_string()].concat()
            } else {
                [STRING.to_string(), string, STRING.to_string()].concat()
            }
        }
        RedisElement::List(list) => parse_list_and_set(list),
        RedisElement::Set(set) => parse_list_and_set(Vec::from_iter(set)),
        RedisElement::Nil => NIL.to_string(),
        RedisElement::SimpleString(string) => string,
    }
}

/// Intenta convertir un `String` en un número `i64`.
///
/// En caso de que no se pueda convertir, devuelve un error representado como `String`.
///
/// De otro modo, retorna el número representado como `i64`.
///
/// # Arguments
///
/// * `string` - String a convertir.
fn convert_to_number(string: String) -> Result<i64, String> {
    let number = string.parse::<i64>();
    match number {
        Ok(number) => Ok(number),
        Err(_) => Err("Error intentando convertir en número".to_string()),
    }
}

/// Parsea un `Vec<String>` al formato correspondiente para mostrar en el html.
///
/// Retorna un `String` con el formato correspondiente.
///
/// # Arguments
///
/// * `vector` - Vector a parsear.
fn parse_list_and_set(vector: Vec<String>) -> String {
    let mut count = 1;
    let mut string = "".to_string();
    if vector.is_empty() {
        return EMPTY_LIST_SET.to_string();
    }
    for element in vector {
        string = [
            string,
            count.to_string(),
            START_LIST.to_string(),
            element,
            END_LIST.to_string(),
        ]
        .concat();
        count += 1;
    }
    string
}
