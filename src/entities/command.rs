use std::collections::HashSet;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Command {
    Ping,
    Copy {
        key_origin: String,
        key_destination: String,
    },
    Get {
        key: String,
    },
    Set {
        key: String,
        value: String,
    },
    Del {
        keys: Vec<String>,
    },
    Exists {
        keys: Vec<String>,
    },
    Rename {
        key_origin: String,
        key_destination: String,
    },
    Incrby {
        key: String,
        increment: u32,
    },
    Decrby {
        key: String,
        decrement: u32,
    },
    Getdel {
        key: String,
    },
    Append {
        key: String,
        value: String,
    },
    Dbsize,
    Lpush {
        key: String,
        value: Vec<String>,
    },
    Lindex {
        key: String,
        index: i32,
    },
    Llen {
        key: String,
    },
    Sadd {
        key: String,
        values: HashSet<String>,
    },
    Scard {
        key: String,
    },
    Sismember {
        key: String,
        value: String,
    },
    Getset {
        key: String,
        value: String,
    },
    Mget {
        keys: Vec<String>,
    },
    Mset {
        key_values: Vec<(String, String)>,
    },
}