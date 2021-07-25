use crate::protocol::decode::decode;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use crate::protocol::type_data::TypeData;

/// Iterador de líneas, enviadas por Redis, que el servidor lee.
pub struct LinesIterator<'a> {
    input: &'a mut BufReader<TcpStream>,
}

impl<'a> LinesIterator<'a> {
    /// Crea un iterador nuevo.
    ///
    /// # Arguments
    ///
    /// * `input` - `BufReader<TcpStream>`.
    pub fn new(input: &'a mut BufReader<TcpStream>) -> Self {
        let input = input;
        Self { input }
    }
}

impl Iterator for LinesIterator<'_> {
    type Item = TypeData;

    /// Función que implementa el `next`, para pedir una nueva línea.
    ///
    /// Mientras se lee una linea, se intenta decodificar.
    /// En el caso de que no se pueda, se pide la siguiente línea.
    ///
    /// Si se logró decodificar correctamente, se retorna la línea decodificada como `Some(line)`.
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let mut buf = String::new();
        while self.input.read_line(&mut buf).ok()? != 0 {
            if let Ok(result) = decode(buf.as_bytes(), 0) {
                let (data, _) = result;
                return Some(data);
            }
        }
        Some(TypeData::Error("Se ha producido un error".to_string()))
    }
}
