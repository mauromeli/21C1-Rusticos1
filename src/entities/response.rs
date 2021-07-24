use crate::entities::redis_element::RedisElement;
use std::sync::mpsc::Receiver;

#[allow(dead_code)]
#[derive(Debug)]
/// Tipos de respuesta del servidor
pub enum Response {
    /// Respuesta de una linea
    Normal(RedisElement),
    /// Respuesta como flujo de datos
    Stream(Receiver<RedisElement>),
    /// Error de comando
    Error(String),
}
