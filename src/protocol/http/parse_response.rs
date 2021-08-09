use crate::entities::redis_element::RedisElement;
use std::iter::FromIterator;

const INTEGER: &str = "(integer) ";
const STRING: &str = "\"";
const START_LIST: &str = ") \"";
const END_LIST: &str = "\" \n";
const NIL: &str = "(nil)";
const EMPTY_LIST_SET: &str = "(empty list or set)";

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

fn convert_to_number(string: String) -> Result<i64, String> {
    let number = string.parse::<i64>();
    match number {
        Ok(number) => Ok(number),
        Err(_) => Err("Error intentando convertir en número".to_string()),
    }
}

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
