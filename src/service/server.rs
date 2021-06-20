use crate::config::server_config::Config;
use crate::entities::command::Command;
use crate::service::command_generator::generate;
use crate::service::redis::Redis;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

#[derive(Debug)]
pub struct Server {
    redis: Redis,
    config: Config,
}

impl Server {
    #[allow(dead_code)]
    pub fn new(config: Config) -> Self {
        let redis = Redis::new();

        Self { redis, config }
    }

    pub fn serve(self) {
        let address = "0.0.0.0:".to_owned() + self.config.clone().get_port().as_str();
        self.server_run(&address).unwrap();
    }

    fn server_run(self, address: &str) -> std::io::Result<()> {
        let listener = TcpListener::bind(address)?;
        let (db_sender, db_receiver) = mpsc::channel();

        self.db_thread(db_receiver);

        while let connection = listener.accept()? {
            let (client, _) = connection;
            let db_sender_clone = db_sender.clone();
            let _ = thread::spawn(move || {
                //TODO: Mover
                Server::client_handler(client, db_sender_clone)
                //TODO: Mover
            });
        }
        Ok(())
    }

    fn client_handler(client: TcpStream, db_sender_clone: Sender<(Command, Sender<String>)>) {
        let client_input: TcpStream = client.try_clone().unwrap();
        let client_output: TcpStream = client;
        let input = BufReader::new(client_input);
        let mut output = client_output;
        let mut lines = input.lines();

        // iteramos las lineas que recibimos de nuestro cliente
        while let Some(request) = lines.next() {
            let (client_sender, client_receiver): (Sender<String>, Receiver<String>) =
                mpsc::channel();

            //TODO: Agregar decode
            let mut vector: Vec<String> = vec![];
            for string in request.unwrap().split(" ") {
                vector.push(string.to_string())
            }
            //TODO: FIN Agregar decode

            let command = generate(vector);
            let output_response;

            // TODO: Agregar forma de escritura
            match command {
                Ok(command) => {
                    let _ = db_sender_clone.send((command, client_sender));
                    let response = client_receiver.recv();
                    output_response = response.unwrap() + "\n";
                }
                _ => {
                    output_response = command.err().unwrap() + "\n";
                }
            };

            let _ = output.write(output_response.as_ref());
        }
    }

    fn db_thread(mut self, db_receiver: Receiver<(Command, Sender<String>)>) {
        let _ = thread::spawn(move || {
            while let msg = db_receiver.recv() {
                if msg.is_err() {
                    panic!();
                }

                let (command, sender): (Command, Sender<String>) = msg.unwrap();
                let redis_response = self.redis.execute(command);
                //TODO: Encode RedisResponse
                let _ = sender.send(redis_response.unwrap().to_string());
            }
        });
    }
}
