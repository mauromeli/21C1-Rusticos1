use proyecto_taller_1::run_redis;
use std::thread;
use std::time::Duration;

extern crate redis;

#[ignore]
#[test]
fn test_set_then_get() {
    let _redis_thread = thread::spawn(move || {
        run_redis(vec![]).unwrap();
    });

    let client = redis::Client::open("redis://localhost:8080/").unwrap();
    let mut con = client.get_connection().unwrap();

    let _set: () = redis::cmd("SET")
        .arg("key")
        .arg(42)
        .query(&mut con)
        .unwrap();
    let get: i32 = redis::cmd("GET").arg("key").query(&mut con).unwrap();

    assert_eq!(get, 42);
}

#[ignore]
#[test]
fn test_expire_key() {
    let _redis_thread = thread::spawn(move || {
        run_redis(vec![]).unwrap();
    });

    let client = redis::Client::open("redis://localhost:8080/").unwrap();
    let mut con = client.get_connection().unwrap();

    let _set: () = redis::cmd("SET")
        .arg("key")
        .arg(42)
        .query(&mut con)
        .unwrap();
    let _expire: () = redis::cmd("EXPIRE")
        .arg("key")
        .arg(1)
        .query(&mut con)
        .unwrap();

    thread::sleep(Duration::from_secs(1));
    let exists: u8 = redis::cmd("EXISTS").arg("key").query(&mut con).unwrap();

    assert_eq!(exists, 0);
}

#[ignore]
#[test]
fn test_set_150_keys() {
    let _redis_thread = thread::spawn(move || {
        run_redis(vec![]).unwrap();
    });

    let client = redis::Client::open("redis://localhost:8080/").unwrap();
    let mut con = client.get_connection().unwrap();

    for i in 0..150 {
        let key = "key".to_owned() + &i.to_string();
        let _set: () = redis::cmd("SET").arg(&key).arg(i).query(&mut con).unwrap();
    }

    for i in 0..150 {
        let key = "key".to_owned() + &i.to_string();
        let get: u8 = redis::cmd("GET").arg(&key).query(&mut con).unwrap();
        assert_eq!(get, i);
    }
}

#[ignore]
#[test]
fn test_lpush_lpop() {
    let _redis_thread = thread::spawn(move || {
        run_redis(vec![]).unwrap();
    });

    let client = redis::Client::open("redis://localhost:8080/").unwrap();
    let mut con = client.get_connection().unwrap();

    let _lpush: () = redis::cmd("LPUSH")
        .arg("list")
        .arg(1)
        .arg(2)
        .arg(3)
        .query(&mut con)
        .unwrap();

    let lpop: u8 = redis::cmd("LPOP").arg("list").query(&mut con).unwrap();
    assert_eq!(lpop, 3);

    let lpop: u8 = redis::cmd("LPOP").arg("list").query(&mut con).unwrap();
    assert_eq!(lpop, 2);

    let lpop: u8 = redis::cmd("LPOP").arg("list").query(&mut con).unwrap();
    assert_eq!(lpop, 1);
}

#[ignore]
#[test]
fn test_publish() {
    let _redis_thread = thread::spawn(move || {
        run_redis(vec![]).unwrap();
    });

    let client = redis::Client::open("redis://localhost:8080").unwrap();
    let mut con = client.get_connection().unwrap();

    let publish: Result<(), _> = redis::cmd("PUBLISH")
        .arg("channel")
        .arg("test-msg")
        .query(&mut con);

    assert!(publish.is_ok());
}

#[ignore]
#[test]
fn test_unsubscribe() {
    let _redis_thread = thread::spawn(move || {
        run_redis(vec![]).unwrap();
    });

    let client = redis::Client::open("redis://localhost:8080").unwrap();
    let mut con = client.get_connection().unwrap();
    let mut pubsub = con.as_pubsub();
    let unsub = pubsub.unsubscribe("channel");

    assert!(unsub.is_ok());
}

#[ignore]
#[test]
fn test_info_shows_connected_clients() {
    let _redis_thread = thread::spawn(move || {
        run_redis(vec![]).unwrap();
    });

    let client = redis::Client::open("redis://localhost:8080/").unwrap();
    let mut con = client.get_connection().unwrap();

    let info: i32 = redis::cmd("INFO")
        .arg("connectedclients")
        .query(&mut con)
        .unwrap();
    assert_eq!(info, 1);
}
