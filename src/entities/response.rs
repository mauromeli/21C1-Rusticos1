use std::collections::HashSet;
use std::fmt;
use crate::entities::redis_element::RedisElement;
use std::sync::mpsc::Receiver;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Response {
    Normal(RedisElement),
    Stream(Receiver<RedisElement>)
}