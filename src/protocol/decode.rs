use crate::protocol::type_data::TypeData;

/// Longitud del `\r\n`.
const CRLF: usize = 2;

///Decodifica el comando recibido desde redis-cli.
///
/// En caso de que el comando esté incompleto, devuelve un error de tipo `String`.
///
/// De otro modo, retorna un `TypeData` que representa a los bytes decodificados y un `usize`, que indica la posición del último byte que se decodificó.
/// # Arguments
///
/// * `bytes` - Comando representado en bytes
/// * `start` - Posición desde donde se debe comenzar a decodificar los bytes
pub fn decode(bytes: &[u8], start: usize) -> std::result::Result<(TypeData, usize), String> {
    if !size_ok(bytes, start) {
        return Err("Error comando incompleto".to_string());
    }
    match bytes[start] as char {
        '+' => {
            if let Ok((string, final_index)) = parse(bytes, start + 1) {
                Ok((TypeData::String(string), final_index))
            } else {
                Err("Error parseando el comando enviado".to_string())
            }
        }
        '-' => {
            if let Ok((error, final_index)) = parse(bytes, start + 1) {
                return Ok((TypeData::Error(error), final_index));
            }
            Err("Error parseando el comando enviado".to_string())
        }
        ':' => {
            if let Ok((integer, final_index)) = parse(bytes, start + 1) {
                return Ok((
                    TypeData::Integer(integer.parse::<i64>().unwrap()),
                    final_index,
                ));
            }
            Err("Error parseando el comando enviado".to_string())
        }

        '$' => {
            if let Ok((bulk_len, final_index)) = parse(bytes, start + 1) {
                let length = bulk_len.parse::<usize>().unwrap();
                if !size_ok(bytes, final_index) {
                    return Err("Error parseando el comando enviado".to_string());
                }
                let bulk =
                    String::from_utf8(bytes[final_index..length + final_index].to_vec()).unwrap();
                return Ok((TypeData::BulkString(bulk), length + final_index + CRLF));
            }
            Err("Error parseando el comando enviado".to_string())
        }
        '*' => {
            if let Ok((array_len, mut final_index)) = parse(bytes, start + 1) {
                let length = array_len.parse::<usize>().unwrap();
                let mut array: Vec<TypeData> = Vec::new();

                for _ in 0..length {
                    match decode(bytes, final_index) {
                        Ok((element, final_pos)) => {
                            array.push(element);
                            final_index = final_pos;
                        }
                        Err(e) => return Err(e),
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
                let bulk = String::from_utf8(bytes[0..length - CRLF].to_vec()).unwrap();
                return Ok((TypeData::BulkString(bulk), length));
            }
            Err("Error parseando el comando enviado".to_string())
        }
    }
}

///Parsea los bytes desde la posición indicada hasta encontrar el primer `/r/n`.
///
/// En caso de que el comando (representado en bytes) esté mal formado, devuelve un error de tipo `String`.
///
/// De otro modo, retorna un `String` que representa a los bytes decodificados y un `usize`, que indica la posición del último byte que se decodificó.
/// # Arguments
///
/// * `bytes` - Comando representado en bytes
/// * `pos` - Posición desde donde se debe comenzar a decodificar los bytes
pub fn parse(bytes: &[u8], pos: usize) -> std::result::Result<(String, usize), String> {
    let vector = Vec::from(&bytes[pos..]);
    if let Ok(string) = String::from_utf8(vector) {
        if let Some(index) = string.find("\r\n") {
            if let Ok(string) = String::from_utf8((bytes[pos..index + pos]).to_vec()) {
                return Ok((string, index + pos + CRLF));
            }
        }
    }
    Err("Error parseando el comando recibido".to_string())
}

/// Pregunta si la longitud de los bytes a codificar está bien.
///
/// En caso de que `pos` sea mayor o igual a la longitud de `bytes` (convertidos en `String`), devuelve `false`.
///
/// De otro modo, retorna `true`.
/// # Arguments
///
/// * `bytes` - Comando representado en bytes
/// * `pos` - Posición desde donde se debe comenzar a decodificar los bytes
pub fn size_ok(bytes: &[u8], pos: usize) -> bool {
    let vector = Vec::from(bytes);
    if let Ok(string) = String::from_utf8(vector) {
        if string.len() <= pos {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use crate::protocol::decode::{decode};
    use crate::protocol::type_data::TypeData;

    #[test]
    fn test_decode_string() {
        let bytes = "+OK\r\n";
        let result = decode(bytes.as_bytes(), 0).unwrap().0;
        assert_eq!(result, TypeData::String("OK".to_string()))
    }

    #[test]
    fn test_decode_error() {
        let bytes = "-Error message\r\n";
        assert_eq!(
            decode(bytes.as_bytes(), 0).ok().unwrap().0,
            TypeData::Error("Error message".to_string())
        )
    }

    #[test]
    fn test_decode_integer() {
        let bytes = ":1000\r\n";
        assert_eq!(
            decode(bytes.as_bytes(), 0).ok().unwrap().0,
            TypeData::Integer(1000)
        )
    }

    #[test]
    fn test_decode_bulk_string() {
        let bytes = "$6\r\nfoobar\r\n";
        assert_eq!(
            decode(bytes.as_bytes(), 0).ok().unwrap().0,
            TypeData::BulkString("foobar".to_string())
        )
    }

    #[test]
    fn test_decode_bulk_empty_string() {
        let bytes = "$0\r\n\r\n";
        assert_eq!(
            decode(bytes.as_bytes(), 0).ok().unwrap().0,
            TypeData::BulkString("".to_string())
        )
    }

    #[test]
    fn test_decode_array() {
        let bytes = "*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n";

        let array = vec![
            TypeData::BulkString("foo".to_string()),
            TypeData::BulkString("bar".to_string()),
        ];
        assert_eq!(
            decode(bytes.as_bytes(), 0).ok().unwrap().0,
            TypeData::Array(array)
        )
    }

    #[test]
    fn test_decode_array_bulkstring() {
        let bytes = "*3\r\n$3\r\nset\r\n$5\r\nmykey\r\n$1\r\n1\r\n";
        let mut vector = Vec::new();
        vector.push(TypeData::BulkString("set".to_string()));
        vector.push(TypeData::BulkString("mykey".to_string()));
        vector.push(TypeData::BulkString("1".to_string()));
        assert_eq!(
            decode(bytes.as_bytes(), 0).ok().unwrap().0,
            TypeData::Array(vector)
        )
    }
}
