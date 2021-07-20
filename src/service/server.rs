use crate::config::server_config::Config;
use crate::entities::command::Command;
use crate::entities::log::Log;
use crate::entities::log_level::LogLevel;
use crate::service::command_generator::generate;
use crate::service::logger::Logger;
use crate::service::redis::Redis;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

static STORE_TIME_SEC: u64 = 120;

#[derive(Debug)]
pub struct Server {
    redis: Redis,
    config: Config,
    log_sender: Sender<Log>,
}

impl Server {
    #[allow(dead_code)]
    pub fn new(config: Config) -> Self {
        let (log_sender, log_receiver): (Sender<Log>, Receiver<Log>) = mpsc::channel();

        let redis = Redis::new(log_sender.clone());

        let logger = Logger::new(log_receiver, config.get_logfile());
        logger.log();

        Self {
            redis,
            config,
            log_sender,
        }
    }

    pub fn serve(mut self) {
        // load db
        let command = Command::Load {
            path: self.config.get_dbfilename(),
        };
        let _ = self.redis.execute(command);
        // endload db

        let address = "0.0.0.0:".to_owned() + self.config.get_port().as_str();
        let sender = self.log_sender.clone();
        let _ = sender.send(Log::new(
            LogLevel::Debug,
            line!(),
            column!(),
            file!().to_string(),
            "=======Server Start Running======".to_string(),
        ));
        self.server_run(&address);
        let _ = sender.send(Log::new(
            LogLevel::Debug,
            line!(),
            column!(),
            file!().to_string(),
            "=======Server Stop Running======".to_string(),
        ));
    }

    fn server_run(self, address: &str) {
        let listener = TcpListener::bind(address).expect("Could not bind");
        let (db_sender, db_receiver) = mpsc::channel();

        let log_sender = self.log_sender.clone();
        let timeout = self.config.get_timeout();

        let db_filename = self.config.get_dbfilename();
        let db_sender_maintenance = db_sender.clone();

        //Todo: Agregar el handler.
        let _ =
            thread::spawn(move || Server::maintenance_thread(db_filename, db_sender_maintenance));

        self.db_thread(db_receiver);

        while let Ok(connection) = listener.accept() {
            let _ = log_sender.send(Log::new(
                LogLevel::Info,
                line!(),
                column!(),
                file!().to_string(),
                "=======New Client Connected======".to_string(),
            ));

            let (client, _) = connection;
            if timeout != 0 {
                client
                    .set_read_timeout(Option::from(Duration::from_secs(timeout)))
                    .expect("Could not set timeout");
            }
            let db_sender_clone = db_sender.clone();
            //TODO: Handler client. encolar en vector booleano compartido para finalizar hilos.
            let _ = thread::spawn(move || Server::client_handler(client, db_sender_clone));
        }
    }

    #[allow(clippy::while_let_on_iterator)]
    fn client_handler(client: TcpStream, db_sender_clone: Sender<(Command, Sender<String>)>) {
        let client_input: TcpStream = client.try_clone().unwrap();
        let client_output: TcpStream = client;
        let input = BufReader::new(client_input);
        let mut output = client_output;
        let mut lines = input.lines();

        // iteramos las lineas que recibimos de nuestro cliente
        while let Some(request) = lines.next() {
            //TODO: Wrappear esto a una func -> Result
            let (client_sndr, client_rcvr): (Sender<String>, Receiver<String>) = mpsc::channel();

            if request.is_err() {
                break;
            }
            //TODO: Agregar decode
            let mut vector: Vec<String> = vec![];
            for string in request.unwrap().split_whitespace() {
                vector.push(string.to_string())
            }
            //TODO: FIN Agregar decode

            let command = generate(vector);
            let output_response;

            // TODO: Agregar forma de escritura por cada tipo.
            match command {
                Ok(command) => {
                    let _ = db_sender_clone.send((command, client_sndr));
                    let response = client_rcvr.recv();
                    output_response = response.unwrap() + "\n";
                }
                _ => {
                    output_response = command.err().unwrap() + "\n";
                }
            };

            let _ = output.write(output_response.as_ref());
            //TODO: Ver que hacer con el error.
        }

        //TODO: flag = false
    }

    fn db_thread(mut self, db_receiver: Receiver<(Command, Sender<String>)>) {
        let _ = thread::spawn(move || {
            while let Ok((command, sender)) = db_receiver.recv() {
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

    //TODO: Return Result. -> Result<(), std::io::Error>
    fn maintenance_thread(file: String, db_receiver: Sender<(Command, Sender<String>)>) {
        loop {
            let (client_sndr, client_rcvr): (Sender<String>, Receiver<String>) = mpsc::channel();
            let command = Command::Store {
                path: file.to_string(),
            };

            /*
            sender.send("hola")?;
            client_recv.recv()?;
            Ok(())

            */
            // Todo: Ver que pasa con  los errores.
            let _ = db_receiver.send((command, client_sndr));
            let _ = client_rcvr.recv();

            thread::sleep(Duration::from_secs(STORE_TIME_SEC));
        }
    }
}
