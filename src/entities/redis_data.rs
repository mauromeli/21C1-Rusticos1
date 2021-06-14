use crate::entities::redis_element::RedisElement;

#[derive(Debug, Clone)]
pub struct RedisData {
    pub element: RedisElement,
    pub timeout: u32,
}

impl RedisData {
    pub fn new(element: RedisElement) -> Self {
        RedisData {
            element: element,
            timeout: 0,
        }
    }
}
