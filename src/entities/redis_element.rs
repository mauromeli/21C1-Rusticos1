use std::collections::HashSet;
use std::fmt;

#[allow(dead_code)]
#[derive(Debug)]
pub enum RedisElement {
    String(String),
    Set(HashSet<String>),
    List(Vec<String>),
}

impl fmt::Display for RedisElement {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RedisElement::String(string) => write!(fmt, "{}", string)?,
            RedisElement::Set(set) => {
                write!(fmt, "{")?;
                for element in set.iter() {
                    write!(fmt, " {}", element)?;
                }
                write!(fmt, " }")?;
            }
            RedisElement::List(list) => {
                write!(fmt, "[")?;
                for element in list {
                    write!(fmt, " {}", element)?;
                }
                write!(fmt, " ]")?;
            }
        }
        Ok(())
    }
}
