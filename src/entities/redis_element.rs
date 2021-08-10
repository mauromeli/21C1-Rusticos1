use std::collections::HashSet;
use std::fmt;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
/// RedisElement: Enum usado para representar los tipos posibles a ser almacenados en nuestra base de
/// datos redis.
pub enum RedisElement {
    /// Representa los tipos de dato String de Redis
    String(String),
    /// Representa los tipos de dato String especiales de Redis
    SimpleString(String),
    /// Representa los tipos de dato Set de Redis
    Set(HashSet<String>),
    /// Representa los tipos de dato List de Redis
    List(Vec<String>),
    /// Representa los tipos de dato Nil de Redis
    Nil,
}

impl fmt::Display for RedisElement {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RedisElement::String(s) => write!(fmt, "{}", s.replace(" - ", "-"))?,
            RedisElement::Set(set) => {
                write!(fmt, "{{")?;

                let mut set = set.iter();
                write!(fmt, "{}", set.next().unwrap().replace(" - ", "-"))?;
                for element in set {
                    write!(fmt, " - {}", element.replace(" - ", "-"))?;
                }
                write!(fmt, "}}")?;
            }
            RedisElement::List(list) => {
                if list.is_empty() {
                    write!(fmt, "[]")?;
                } else {
                    write!(fmt, "[")?;

                    let mut list = list.iter();
                    write!(fmt, "{}", list.next().unwrap().replace(" - ", "-"))?;
                    for element in list {
                        write!(fmt, " - {}", element.replace(" - ", "-"))?;
                    }
                    write!(fmt, "]")?;
                }
            }
            RedisElement::Nil => {
                write!(fmt, "(nil)")?;
            }
            RedisElement::SimpleString(s) => write!(fmt, "{}", s.replace(" - ", "-"))?,
        }
        Ok(())
    }
}

impl From<&str> for RedisElement {
    fn from(s: &str) -> Self {
        if s.starts_with("(nil)") {
            RedisElement::Nil
        } else if s.starts_with('{') && s.ends_with('}') {
            let mut set: HashSet<String> = HashSet::new();
            let s = s.strip_prefix('{').unwrap().strip_suffix('}').unwrap();

            for element in s.split(" - ") {
                set.insert(element.to_string());
            }
            RedisElement::Set(set)
        } else if s.starts_with('[') && s.ends_with(']') {
            let mut list: Vec<String> = Vec::new();
            let s = s.strip_prefix('[').unwrap().strip_suffix(']').unwrap();

            for element in s.split(" - ") {
                list.push(element.to_string());
            }
            RedisElement::List(list)
        } else {
            RedisElement::String(s.to_string())
        }
    }
}
