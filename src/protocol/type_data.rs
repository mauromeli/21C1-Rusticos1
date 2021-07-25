#[derive(Debug, Clone, PartialEq)]
/// Representa el tipo de dato para decodificar/codificar, utilizando el protocolo RESP.
pub enum TypeData {
    /// Representa la cadena simple (aquella que tiene como primer byte `+`)
    String(String),
    /// Representa los errores (aquellos que tiene como primer byte `-`)
    Error(String),
    /// Representa los enteros (aquellos que tiene como primer byte `:`)
    Integer(i64),
    /// Representa Bulk Strings (aquellos que tiene como primer byte `$`)
    BulkString(String),
    /// Representa las matrices (aquellas que tiene como primer byte `*`)
    Array(Vec<TypeData>),
    /// Representa el nulo (`*-1\r\n`)
    Nil,
}