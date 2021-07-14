use std::sync::mpsc::Receiver;
use crate::entities::redis_element::RedisElement;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Response {
    Normal(RedisElement),
    Stream(Receiver<RedisElement>),
    Error(String)
}
