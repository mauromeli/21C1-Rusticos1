use crate::service::redis::Redis;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, BufReader, BufRead};
use std::sync::mpsc;
use std::thread;

#[derive(Debug)]
pub struct Server {
    redis: Redis,
}

impl Server {
    #[allow(dead_code)]
    pub fn new() -> Self {
        //TODO: Add config to constructor
        let redis = Redis::new();

        Self { redis: redis }
    }

    pub fn serve(self) {
        //TODO: leer de config
        let address = "0.0.0.0:".to_owned() + "8080";
        server_run(&address).unwrap();
    }
}

//TODO: Hay forma de meterlo en el mismo struct?
fn server_run(address: &str) -> std::io::Result<()> {
    let listener = TcpListener::bind(address)?;
    // accept devuelve una tupla (TcpStream, std::net::SocketAddr)
    while let connection = listener.accept()? {
        let (client, _) = connection;
        let handler = thread::spawn(move || {
            let mut client_stream: TcpStream = client;
            // TcpStream implementa el trait Read, así que podemos trabajar como si fuera un archivo
            handle_client(&mut client_stream);
        });
    }
    Ok(())
}

fn handle_client(stream: &mut dyn Read) -> std::io::Result<()> {
    let reader = BufReader::new(stream);
    let mut lines = reader.lines();
    // iteramos las lineas que recibimos de nuestro cliente

    /*let (dbSender, dbReceiver) = mpsc::channel();
    let hiloDB = thread::spawn(move ||
        {
            while let msg = dbReceiver.recv() {
                //TODO: Ver que hacer en DB
            }
        }
    );*/

    while let Some(client) = lines.next() {
        let handler = thread::spawn(move || {
            println!("{:?}", client.unwrap());
        });
    }

    Ok(())
}
