use std::collections::HashSet;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Command {
    // Server
    Ping,
    Dbsize,
    // Strings
    Get {
        key: String,
    },
    Set {
        key: String,
        value: String,
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
    // Keys
    Copy {
        key_origin: String,
        key_destination: String,
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
    // List
    Lindex {
        key: String,
        index: i32,
    },
    Llen {
        key: String,
    },
    Lpush {
        key: String,
        value: Vec<String>,
    },
    Lpushx {
        key: String,
        value: Vec<String>,
    },
    /*Lrem {
        key: String,
    },*/
    // Sets
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
    Smembers {
        key: String,
    },
    Srem {
        key: String,
        values: HashSet<String>,
    },
}
