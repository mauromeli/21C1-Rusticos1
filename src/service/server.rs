use crate::service::redis::Redis;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, BufReader, BufRead, Write, BufWriter};
use std::sync::mpsc;
use std::thread;
use crate::service::command_generator::generate;
use std::sync::mpsc::{Sender, Receiver};
use crate::entities::command::Command;

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

    pub fn serve(mut self) {
        //TODO: leer de config
        let address = "0.0.0.0:".to_owned() + "8080";
        self.server_run(&address).unwrap();
    }


    //TODO: Hay forma de meterlo en el mismo struct?
    fn server_run(mut self, address: &str) -> std::io::Result<()> {
        let listener = TcpListener::bind(address)?;

        let (dbSender, dbReceiver) = mpsc::channel();

        let hiloDB = thread::spawn(move ||
            {
                while let msg = dbReceiver.recv() {
                    if msg.is_err() {
                        panic!();
                    }
                    let (command, sender): (Command, Sender<String>) = msg.unwrap();
                    //TODO: Ver que hacer en DB
                    let redisResponse = self.redis.execute(command);

                    println!("{:?}", redisResponse);
                    sender.send(redisResponse.unwrap().to_string());

                    //TODO: Encode RedisResponse
                }
            }
        );

        while let connection = listener.accept()? {
            let (client, _) = connection;
            let dbSender_clone = dbSender.clone();
            let handler = thread::spawn(move || {
                let client_input: TcpStream = client.try_clone().unwrap();
                let client_output: TcpStream = client;

                //handle_client(&mut client_stream);
                //TODO: Mover

                let input = BufReader::new(client_input);
                let mut output = client_output;
                let mut lines = input.lines();

                // iteramos las lineas que recibimos de nuestro cliente
                while let Some(request) = lines.next() {
                    //println!("{:?}", request.unwrap());

                    let (clientSender, clientReceiver): (Sender<String>, Receiver<String>) = mpsc::channel();

                    //TODO: Agregar decode
                    let mut vector: Vec<String> = vec![];
                    for string in request.unwrap().split(" ") {
                        vector.push(string.to_string())
                    }
                    println!("{:?}", vector);
                    let command = generate(vector);

                    println!("{:?}", command);

                    match command {
                        Ok(command) => {
                            dbSender_clone.send((command, clientSender));
                        }
                        _ => {
                            output.write("Error".to_string().as_ref());
                        }
                    };

                    let response = clientReceiver.recv();
                    println!("{:?}",response);
                    let nbytes = output.write(response.unwrap().as_bytes());
                    output.write("\n".as_bytes());
                    println!("nbytes: {:?}", nbytes);
                }

                //TODO: Mover
            });
        }
        Ok(())
    }
}
