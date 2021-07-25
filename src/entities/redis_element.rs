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
            let s = s.strip_prefix("{").unwrap().strip_suffix("}").unwrap();

            for element in s.split(" - ") {
                set.insert(element.to_string());
            }
            RedisElement::Set(set)
        } else if s.starts_with('[') && s.ends_with(']') {
            let mut list: Vec<String> = Vec::new();
            let s = s.strip_prefix("[").unwrap().strip_suffix("]").unwrap();

            for element in s.split(" - ") {
                list.push(element.to_string());
            }
            RedisElement::List(list)
        } else {
            RedisElement::String(s.to_string())
        }
    }
}
