use crate::protocol::decode::{decode, TypeData};
use std::io::{BufRead, BufReader};
use std::net::TcpStream;

pub struct LinesIterator<'a> {
    input: &'a mut BufReader<TcpStream>,
}

impl<'a> LinesIterator<'a> {
    pub fn new(input: &'a mut BufReader<TcpStream>) -> Self {
        let input = input;
        Self { input }
    }
}

impl Iterator for LinesIterator<'_> {
    type Item = TypeData;

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
