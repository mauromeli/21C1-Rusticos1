use std::io::Result;

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
pub fn decode(bytes: &[u8], start: usize) -> Result<(TypeData, usize)> {
    match bytes[start] as char{
        '+' => {
            let (string, final_index) = parse(bytes, start+1);
            Ok((TypeData::String(string), final_index))
        }
        '-' => {
            let (error, final_index) = parse(bytes, start+1);
            Ok((TypeData::Error(error), final_index))
        }
        ':' => {
            let (integer, final_index) = parse(bytes, start+1);
            Ok((TypeData::Integer(integer.parse::<i64>().unwrap()), final_index))
        }

        '$' => {
            let (bulk_len, final_index) = parse(bytes, start+1);
            let length = bulk_len.parse::<usize>().unwrap();
            let bulk = String::from_utf8(bytes[final_index..length+final_index].to_vec()).unwrap();
            Ok((TypeData::BulkString(bulk), length+final_index+CRLF))
        }

        '*' => {
            let (array_len, mut final_index) = parse(bytes, start+1);
            let length = array_len.parse::<usize>().unwrap();
            let mut array : Vec<TypeData> = Vec::new();

            for _ in 0..length {
                let result = decode(bytes, final_index);
                let element = result.as_ref().unwrap().clone().0;
                let final_pos = result.as_ref().unwrap().clone().1;
                array.push(element);
                final_index = final_pos;
            }
            Ok((TypeData::Array(array), final_index))
        }

        _ => Ok((TypeData::Nil, 0))
    }
}

#[allow(dead_code)]
pub fn parse(bytes: &[u8], pos: usize) -> (String, usize) {
    let index: Option<usize> = String::from_utf8(Vec::from(&bytes[pos..])).unwrap().find("\r\n");
    (String::from_utf8(bytes[pos..index.unwrap()+pos].to_vec()).unwrap(), index.unwrap()+pos+CRLF)
}

#[test]
fn test_decode_string(){
    let bytes = "+OK\r\n";
    assert_eq!(decode(bytes.as_bytes(), 0).ok().unwrap().0,
               TypeData::String("OK".to_string()))
}

#[test]
fn test_decode_error(){
    let bytes = "-Error message\r\n";
    assert_eq!(decode(bytes.as_bytes(),0).ok().unwrap().0
               , TypeData::Error("Error message".to_string()))
}

#[test]
fn test_decode_integer(){
    let bytes = ":1000\r\n";
    assert_eq!(decode(bytes.as_bytes(), 0).ok().unwrap().0,
               TypeData::Integer(1000))
}

#[test]
fn test_decode_bulk_string(){
    let bytes = "$6\r\nfoobar\r\n";
    assert_eq!(decode(bytes.as_bytes(), 0).ok().unwrap().0,
               TypeData::BulkString("foobar".to_string()))
}

#[test]
fn test_decode_bulk_empty_string(){
    let bytes = "$0\r\n\r\n";
    assert_eq!(decode(bytes.as_bytes(), 0).ok().unwrap().0,
               TypeData::BulkString("".to_string()))
}

#[test]
fn test_decode_array(){
    let bytes = "*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n";

    let array = vec![TypeData::BulkString("foo".to_string()),
                     TypeData::BulkString("bar".to_string())];
    assert_eq!(decode(bytes.as_bytes(), 0).ok().unwrap().0,
               TypeData::Array(array))
}

#[test]
fn test_decode_nil(){
    let bytes = "!3\r\nfoo\r\n";
    assert_eq!(decode(bytes.as_bytes(), 0).ok().unwrap().0,
               TypeData::Nil)
}