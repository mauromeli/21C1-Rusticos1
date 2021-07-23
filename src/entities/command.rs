use crate::entities::info_param::InfoParam;
use crate::entities::pubsub_param::PubSubParam;
use std::collections::HashSet;
use std::time::{Duration, SystemTime};

#[derive(Debug)]
#[allow(dead_code)]
pub enum Command {
    // Server
    Ping,
    Flushdb,
    Dbsize,
    Monitor,
    Info {
        param: InfoParam,
    },

    // System
    Store {
        path: String,
    },
    Load {
        path: String,
    },
    AddClient,
    RemoveClient,

    // Strings
    Get {
        key: String,
    },
    Set {
        key: String,
        value: String,
    },
    Keys {
        pattern: String,
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
    Rename {
        key_origin: String,
        key_destination: String,
    },
    Expire {
        key: String,
        ttl: Duration,
    },
    Expireat {
        key: String,
        ttl: SystemTime,
    },
    Persist {
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

    // pubsub
    Pubsub {
        param: PubSubParam,
    },
    Subscribe {
        channels: Vec<String>,
        //local_address: String,
    },
    Publish {
        channel: String,
        message: String,
    },
    Unsubscribe {
        channels: Vec<String>, //local_address: String,
    },
}

impl Command {
    pub fn as_str(&self) -> &'static str {
        match *self {
            // Server
            Command::Ping => "ping",
            Command::Flushdb => "flushdb",
            Command::Dbsize => "dbsize",
            Command::Monitor => "monitor",
            Command::Info { .. } => "info",

            // Strings
            Command::Append { .. } => "append",
            Command::Decrby { .. } => "decrby",
            Command::Get { .. } => "get",
            Command::Getdel { .. } => "getdel",
            Command::Getset { .. } => "getset",
            Command::Incrby { .. } => "incrby",
            Command::Mget { .. } => "mget",
            Command::Mset { .. } => "mset",
            Command::Set { .. } => "set",
            Command::Strlen { .. } => "strlen",

            // Keys
            Command::Copy { .. } => "copy",
            Command::Del { .. } => "del",
            Command::Exists { .. } => "exists",
            Command::Expire { .. } => "expire",
            Command::Expireat { .. } => "expireat",
            Command::Persist { .. } => "persist",
            Command::Rename { .. } => "rename",
            Command::Keys { .. } => "keys",
            Command::Touch { .. } => "touch",
            Command::Ttl { .. } => "ttl",
            Command::Type { .. } => "type",

            // Lists
            Command::Lindex { .. } => "lindex",
            Command::Llen { .. } => "llen",
            Command::Lpop { .. } => "lpop",
            Command::Lpush { .. } => "lpush",
            Command::Lpushx { .. } => "lpushx",
            Command::Lrange { .. } => "lrange",
            Command::Lrem { .. } => "lrem",
            Command::Lset { .. } => "lset",
            Command::Rpop { .. } => "rpop",
            Command::Rpush { .. } => "rpush",
            Command::Rpushx { .. } => "rpushx",

            // Sets
            Command::Sadd { .. } => "sadd",
            Command::Scard { .. } => "scard",
            Command::Sismember { .. } => "sismember",
            Command::Smembers { .. } => "smember",
            Command::Srem { .. } => "srem",

            // Pubsub
            Command::Pubsub { .. } => "pubsub",
            Command::Subscribe { .. } => "subscribe",
            Command::Publish { .. } => "publish",
            Command::Unsubscribe { .. } => "unsubscribe",

            _ => "",
        }
    }
}