use std::collections::HashSet;
use std::fmt;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum RedisElement {
    String(String),
    Set(HashSet<String>),
    List(Vec<String>),
    Nil,
}

impl fmt::Display for RedisElement {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RedisElement::String(string) => write!(fmt, "{}", string)?,
            RedisElement::Set(set) => {
                write!(fmt, "{{")?;
                for element in set.iter() {
                    write!(fmt, " {}", element)?;
                }
                write!(fmt, " }}")?;
            }
            RedisElement::List(list) => {
                write!(fmt, "[")?;
                for element in list {
                    write!(fmt, " {}", element)?;
                }
                write!(fmt, " ]")?;
            }
            RedisElement::Nil => {
                write!(fmt, "(nil)")?;
            }
        }
        Ok(())
    }
}
