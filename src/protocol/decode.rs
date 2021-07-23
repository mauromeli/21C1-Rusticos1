use std::io::Result;
use std::fmt::Error;

#[warn(dead_code)]
const CRLF: usize = 2;

#[derive(Debug, Clone, PartialEq)]
pub enum TypeData {
    String(String),
    Error(String),
    Integer(i64),
    BulkString(String),
    Array(Vec<TypeData>),
    Nil
}

#[allow(dead_code)]
pub fn decode(bytes: &[u8], start: usize) -> std::result::Result<(TypeData, usize), String> {
    if !size_ok(bytes, start) {
        return Err("Error comando incompleto".to_string());
    }
    match bytes[start] as char {
        '+' => {
            if let Ok((string, final_index)) = parse(bytes, start+1){
                return Ok((TypeData::String(string), final_index));
            } else {
                Err("Error parseando el comando enviado".to_string())
            }
        }
        '-' => {
            if let Ok((error, final_index)) = parse(bytes, start+1) {
                return Ok((TypeData::Error(error), final_index));
            }
            Err("Error parseando el comando enviado".to_string())
        }
        ':' => {
            if let Ok((integer, final_index)) = parse(bytes, start+1) {
                return Ok((TypeData::Integer(integer.parse::<i64>().unwrap()), final_index));
            }
            Err("Error parseando el comando enviado".to_string())
        }

        '$' => {
            if let Ok((bulk_len, final_index)) = parse(bytes, start+1) {
                let length = bulk_len.parse::<usize>().unwrap();
                if !size_ok(bytes, final_index) {
                    return Err("Error parseando el comando enviado".to_string());
                }
                let bulk = String::from_utf8(bytes[final_index..length+final_index].to_vec()).unwrap();
                return Ok((TypeData::BulkString(bulk), length+final_index+CRLF));
            }
            Err("Error parseando el comando enviado".to_string())
        }
        '*' => {
            if let Ok((array_len, mut final_index)) = parse(bytes, start+1) {
                let length = array_len.parse::<usize>().unwrap();
                let mut array : Vec<TypeData> = Vec::new();

                for _ in 0..length {
                    match decode(bytes, final_index) {
                        Ok((element, final_pos)) => {
                            array.push(element);
                            final_index = final_pos;
                        }
                        Err(e) =>  return Err(e),
                    }
                }
                return Ok((TypeData::Array(array), final_index));
            }
            Err("Error parseando el comando enviado".to_string())
        }

        _ => {
            let vector = Vec::from(bytes);
            if let Ok(string) = String::from_utf8(vector) {
                let length = string.len();
                let bulk = String::from_utf8(bytes[0..length-CRLF].to_vec()).unwrap();
                return Ok((TypeData::BulkString(bulk), length));
            }
            Err("Error parseando el comando enviado".to_string())
        }
    }
}

    #[allow(dead_code)]
    pub fn parse(bytes: &[u8], pos: usize) -> std::result::Result<(String, usize), String> {
        let vector = Vec::from(&bytes[pos..]);
        if let Ok(string) = String::from_utf8(vector) {
            if let Some(index) = string.find("\r\n") {
                if let Ok(string) = String::from_utf8((bytes[pos..index+pos]).iter().cloned().collect()) {
                    return Ok((string, index + pos + CRLF));
                }
            }
        }
        return Err("Error parseando el comando recibido".to_string());
    }

pub fn size_ok(bytes: &[u8], pos: usize) -> bool {
    let vector = Vec::from(bytes);
    if let Ok(string) = String::from_utf8(vector) {
        if string.len() <= pos {
            return false;
        }
    }
    return true;
}




mod test {
    use crate::protocol::decode::{decode, TypeData};

    #[test]
    fn test_decode() {
        let bytes = "*1\r\n$7\r\n";
        let result = decode(bytes.as_bytes(), 0);
        assert_eq!(result.is_ok(), true);
        assert_eq!(decode(bytes.as_bytes(), 0).ok().unwrap().0,
                   TypeData::String("OK".to_string()))
    }


    #[test]
    fn test_decode_string() {
        let bytes = "+OK\r\n";
        let result = decode(bytes.as_bytes(), 0).unwrap().0;
        assert_eq!(result,
                   TypeData::String("OK".to_string()))
    }

    #[test]
    fn test_decode_error() {
        let bytes = "-Error message\r\n";
        assert_eq!(decode(bytes.as_bytes(), 0).ok().unwrap().0
                   , TypeData::Error("Error message".to_string()))
    }

    #[test]
    fn test_decode_integer() {
        let bytes = ":1000\r\n";
        assert_eq!(decode(bytes.as_bytes(), 0).ok().unwrap().0,
                   TypeData::Integer(1000))
    }

    #[test]
    fn test_decode_bulk_string() {
        let bytes = "$6\r\nfoobar\r\n";
        assert_eq!(decode(bytes.as_bytes(), 0).ok().unwrap().0,
                   TypeData::BulkString("foobar".to_string()))
    }

    #[test]
    fn test_decode_bulk_empty_string() {
        let bytes = "$0\r\n\r\n";
        assert_eq!(decode(bytes.as_bytes(), 0).ok().unwrap().0,
                   TypeData::BulkString("".to_string()))
    }

    #[test]
    fn test_decode_array() {
        let bytes = "*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n";

        let array = vec![TypeData::BulkString("foo".to_string()),
                         TypeData::BulkString("bar".to_string())];
        assert_eq!(decode(bytes.as_bytes(), 0).ok().unwrap().0,
                   TypeData::Array(array))
    }

    #[test]
    fn test_decode_nil() {
        let bytes = "!3\r\nfoo\r\n";
        assert_eq!(decode(bytes.as_bytes(), 0).ok().unwrap().0,
                   TypeData::Nil)
    }

    #[test]
    fn test_decode_() {
        let bytes = "*3\r\n$3\r\nset\r\n$5\r\nmykey\r\n$1\r\n1\r\n";
        assert_eq!(decode(bytes.as_bytes(), 0).ok().unwrap().0,
                   TypeData::Nil)
    }

}