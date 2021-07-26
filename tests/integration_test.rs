use proyecto_taller_1::run_redis;
extern crate redis;

#[test]
fn test_set_then_get() {
    run_redis(vec![]).unwrap();

    let client = redis::Client::open("redis://localhost:6379/").unwrap();
    assert!(false);
    let mut con = client.get_connection().unwrap();

    let _set: () = redis::cmd("SET")
        .arg("key")
        .arg(42)
        .query(&mut con)
        .unwrap();
    let get: i32 = redis::cmd("GET").arg("key").query(&mut con).unwrap();

    assert_eq!(get, 42);
}

#[test]
fn test_pubsub() {
    run_redis(vec![]).unwrap();

    let client = redis::Client::open("redis://localhost:6379").unwrap();
    let mut con = client.get_connection().unwrap();

    let mut pubsub = con.as_pubsub();
    pubsub.subscribe("channel_1").unwrap();
    pubsub.subscribe("channel_2").unwrap();

    let msg = pubsub.get_message().unwrap();
    let payload: String = msg.get_payload().unwrap();
    println!("channel '{}': {}", msg.get_channel_name(), payload);
}
