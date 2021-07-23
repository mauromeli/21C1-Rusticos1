use crate::config::server_config::Config;
use crate::entities::command::Command;
use crate::service::command_generator::generate;
use crate::service::redis::Redis;
use std::io::{BufRead, BufReader, Write, Lines, Read, Error};
use std::net::{TcpListener, TcpStream};
use std::sync::{mpsc, Arc};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use crate::protocol::parse_data::parse_data;
use std::borrow::BorrowMut;
use crate::protocol::decode::{decode, TypeData};

pub const CRLF: &[u8] = b"\r\n";

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

            let (client, socket_addr) = connection;
            let db_sender_clone = db_sender.clone();

            let _ = thread::spawn(move || Server::client_handler(client, db_sender_clone,
                                                                 socket_addr.to_string()));
        }
        Ok(())
    }

    fn client_handler(client: TcpStream, db_sender_clone: Sender<(Command, Sender<String>, String)>,
                    local_address: String) {
        let client_input: TcpStream = client.try_clone().unwrap();
        let client_output: TcpStream = client;
        let mut input = BufReader::new(client_input);
        let mut output = client_output;
        //let mut lines = input.lines();

        // iteramos las lineas que recibimos de nuestro cliente
        while let Some(line) = LinesIterator::new(&mut input).next() {

            let (client_sndr, client_rcvr): (Sender<String>, Receiver<String>) = mpsc::channel();

            //TODO: Agregar decode
            let vector = parse_data(line);
            //TODO: FIN Agregar decode

            let command = generate(vector);
            let output_response;

            // TODO: Agregar forma de escritura
            match command {
                /*
                Ok(Command::Command) => {
                    //output_response = "*200\r\n".to_string();
                }

                 */

                Ok(command) => {
                    let _ = db_sender_clone.send((command, client_sndr, local_address.clone()));
                    let response = client_rcvr.recv();
                    output_response = response.unwrap() + "\n";
                }
                _ => {
                    output_response = command.err().unwrap() + "\n";
                }
            };

            let _ = output.write(output_response.as_bytes());
        }

    }

    fn db_thread(mut self, db_receiver: Receiver<(Command, Sender<String>, String)>) {
        let _ = thread::spawn(move || {
            while let msg = db_receiver.recv() {
                if msg.is_err() {
                    panic!();
                }

                let (command, sender, local_addr): (Command, Sender<String>, String) = msg.unwrap();
                let redis_response = self.redis.execute(command);
                //TODO: Encode RedisResponse
                let output_response;
                match redis_response {
                    Ok(value) => {
                        output_response = value.to_string();
                    }
                    Err(error_msg) => {
                        output_response = error_msg;
                    }
                };
                let _ = sender.send(output_response);
            }
        });
    }

}

pub struct LinesIterator<'a>{
    input: &'a mut BufReader<TcpStream>
    //input: &'a mut Lines<BufReader<TcpStream>>
}


    impl<'a> LinesIterator<'a> {
        //pub fn new(input: &'a mut Lines<BufReader<TcpStream>>) -> Self {
        pub fn new(input: &'a mut BufReader<TcpStream>) -> Self {
            let input = input;
            Self {input}
        }
    }

    impl Iterator for LinesIterator<'_> {
        type Item = TypeData;

        fn next(&mut self) -> Option<<Self as Iterator>::Item> {
            let mut buf = String::new();
            while self.input.read_line(&mut buf).unwrap() != 0 {
                println!("read: {:?}", buf);
                if let Ok(result) = decode(buf.as_bytes(), 0) {
                    let (data, _) = result;
                    println!("CORTA EJECUCION devuelve: {:?}", data);
                    return Some(data);
                    break
                }
            }
            Some(TypeData::Nil)
        }


        /*
        fn next(&mut self) -> Option<<Self as Iterator>::Item> {
            let mut bytes = Vec::new();
            while let Some(line) = self.input.next() {
                println!("next");
                let read = line.unwrap();
                println!("read: {:?}", read.clone());
                bytes = [bytes, read.as_bytes().to_vec(), "\r\n".as_bytes().to_vec()].concat();
            }
            Some(bytes)
        }


         */

    }

