use crate::protocol::decode::{decode, TypeData};

pub fn parse_data(bytes: &[u8]) -> Vec<String> {
    let (type_data, _) = decode(bytes, 0).unwrap();
    parse_array(type_data).unwrap()
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
        _ => {
            Err("Error comando ingresado".to_string())
        }
    }
}

fn parse_type_data(type_data: TypeData) -> Result<String, String> {
    match type_data {
        TypeData::String(string) => {
            Ok(string)
        }
        TypeData::Integer(integer) => {
            Ok(integer.to_string())
        }
        TypeData::BulkString(bulkstring) => {
            Ok(bulkstring)
        }
        _ => {
            Err("Error tipo de dato".to_string())
        }
    }
}