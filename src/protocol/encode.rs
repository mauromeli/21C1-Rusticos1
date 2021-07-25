use crate::protocol::decode::TypeData;

pub fn encode(data: TypeData) -> Vec<u8> {
    match data {
        TypeData::String(string) => {
            let bytes = [
                "+".to_string().as_bytes(),
                string.as_bytes(),
                "\r\n".as_bytes(),
            ]
            .concat();
            bytes
        }
        TypeData::Error(error) => {
            let bytes = [
                "-".to_string().as_bytes(),
                error.as_bytes(),
                "\r\n".as_bytes(),
            ]
            .concat();
            bytes
        }
        TypeData::Integer(int) => {
            let bytes = [
                ":".to_string().as_bytes(),
                int.to_string().as_bytes(),
                "\r\n".as_bytes(),
            ]
            .concat();
            bytes
        }
        TypeData::BulkString(bulk) => {
            let bytes = [
                "$".to_string().as_bytes(),
                bulk.len().to_string().as_bytes(),
                "\r\n".as_bytes(),
                bulk.as_bytes(),
                "\r\n".as_bytes(),
            ]
            .concat();
            bytes
        }
        TypeData::Array(array) => {
            let mut bytes = [
                "*".to_string().as_bytes(),
                array.len().to_string().as_bytes(),
                "\r\n".as_bytes(),
            ]
            .concat();
            for element in array {
                let result = encode(element.clone());
                bytes = [bytes, result].concat();
            }
            bytes
        }
        TypeData::Nil => {
            let bytes = [
                "$".to_string().as_bytes(),
                "-1".to_string().as_bytes(),
                "\r\n".as_bytes(),
            ].concat();
            bytes
        }
    }
}

#[test]
fn test_encode_string() {
    let bytes = "+OK\r\n".as_bytes();
    assert_eq!(encode(TypeData::String("OK".to_string())), bytes)
}

#[test]
fn test_encode_error() {
    let bytes = "-Error message\r\n".as_bytes();
    assert_eq!(encode(TypeData::Error("Error message".to_string())), bytes)
}

#[test]
fn test_encode_integer() {
    let bytes = ":1000\r\n".as_bytes();
    assert_eq!(encode(TypeData::Integer(1000)), bytes)
}

#[test]
fn test_encode_bulk() {
    let bytes = "$6\r\nfoobar\r\n".as_bytes();
    assert_eq!(encode(TypeData::BulkString("foobar".to_string())), bytes)
}

#[test]
fn test_encode_array() {
    let bytes = "*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n".as_bytes();
    let array = vec![
        TypeData::BulkString("foo".to_string()),
        TypeData::BulkString("bar".to_string()),
    ];
    assert_eq!(encode(TypeData::Array(array)), bytes)
}
