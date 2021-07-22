use std::collections::HashSet;
use std::time::{Duration, SystemTime};

#[derive(Debug)]
#[allow(dead_code)]
pub enum Command {
    // Server
    Ping,
    Flushdb,
    Dbsize,
    Store {
        path: String,
    },
    Load {
        path: String,
    },
    Monitor,

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
    Strlen {
        key: String,
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
    Expire {
        key: String,
        ttl: Duration,
    },
    Expireat {
        key: String,
        ttl: SystemTime,
    },
    Keys {
        pattern: String,
    },
    Persist {
        key: String,
    },
    Rename {
        key_origin: String,
        key_destination: String,
    },
    Sort {
        key: String,
    },
    Touch {
        keys: Vec<String>,
    },
    Ttl {
        key: String,
    },
    Type {
        key: String,
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
    Lpop {
        key: String,
        count: usize,
    },
    Lrange {
        key: String,
        begin: i32,
        end: i32,
    },
    Lrem {
        key: String,
        count: i32,
        element: String,
    },
    Lset {
        key: String,
        index: i32,
        element: String,
    },
    Rpop {
        key: String,
        count: usize,
    },
    Rpush {
        key: String,
        value: Vec<String>,
    },
    Rpushx {
        key: String,
        value: Vec<String>,
    },

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
