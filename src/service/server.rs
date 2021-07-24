use crate::config::server_config::Config;
use crate::entities::command::Command;
use crate::entities::log::Log;
use crate::entities::log_level::LogLevel;
use crate::service::command_generator::generate;
use crate::service::logger::Logger;
use crate::service::redis::Redis;
use std::io;
use std::io::{BufRead, BufReader, Error, ErrorKind, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

static STORE_TIME_SEC: u64 = 120;

#[derive(Debug)]
pub struct Server {
    redis: Redis,
    log_sender: Sender<Log>,
    config: Arc<Mutex<Config>>,
}

impl Server {
    #[allow(dead_code)]
    pub fn new(config: Config) -> io::Result<Self> {
        let (log_sender, log_receiver): (Sender<Log>, Receiver<Log>) = mpsc::channel();

        let config = Arc::new(Mutex::new(config));
        let logger = Logger::new(log_receiver, Arc::clone(&config));
        let redis = Redis::new(log_sender.clone(), Arc::clone(&config));

        logger.log();

        Ok(Self {
            redis,
            log_sender,
            config,
        })
    }

    pub fn serve(mut self) -> Result<(), Box<dyn std::error::Error>> {
        // load db
        let command = Command::Load {
            path: self.config.lock().unwrap().get_dbfilename(),
        };
        let _ = self.redis.execute(command);
        // endload db

        let address = "0.0.0.0:".to_owned() + self.config.lock().unwrap().get_port().as_str();
        let log_sender = self.log_sender.clone();
        log_sender
            .send(Log::new(
                LogLevel::Debug,
                line!(),
                column!(),
                file!().to_string(),
                "=======Server Start Running======".to_string(),
            ))
            .map_err(|_| Error::new(ErrorKind::ConnectionAborted, "Log Sender error"))?;

        self.server_run(&address)?;

        log_sender
            .send(Log::new(
                LogLevel::Debug,
                line!(),
                column!(),
                file!().to_string(),
                "=======Server Stop Running======".to_string(),
            ))
            .map_err(|_| Error::new(ErrorKind::ConnectionAborted, "Log Sender error"))?;
        Ok(())
    }

    fn server_run(self, address: &str) -> io::Result<()> {
        let listener = TcpListener::bind(address)?;
        let (db_sender, db_receiver) = mpsc::channel();

        let log_sender = self.log_sender.clone();
        let timeout = self.config.lock().unwrap().get_timeout();

        let db_filename = self.config.lock().unwrap().get_dbfilename();
        let db_sender_maintenance = db_sender.clone();

        //Todo: Agregar el handler.
        let _: JoinHandle<Result<(), io::Error>> = thread::spawn(move || {
            Server::maintenance_thread(db_filename, db_sender_maintenance)?;
            Ok(())
        });

        self.db_thread(db_receiver);

        while let Ok(connection) = listener.accept() {
            log_sender
                .send(Log::new(
                    LogLevel::Info,
                    line!(),
                    column!(),
                    file!().to_string(),
                    "=======New Client Connected======".to_string(),
                ))
                .map_err(|_| Error::new(ErrorKind::ConnectionAborted, "Log Sender error"))?;

            let (client, _) = connection;
            if timeout != 0 {
                client.set_read_timeout(Option::from(Duration::from_secs(timeout)))?;
            }
            let db_sender_clone = db_sender.clone();
            //TODO: Handler client. encolar en vector booleano compartido para finalizar hilos.

            let _: JoinHandle<Result<(), io::Error>> = thread::spawn(move || {
                Server::client_handler(client, db_sender_clone)?;
                Ok(())
            });
        }
        Ok(())
    }

    #[allow(clippy::while_let_on_iterator)]
    fn client_handler(
        client: TcpStream,
        db_sender_clone: Sender<(Command, Sender<String>)>,
    ) -> io::Result<()> {
        let client_input: TcpStream = client.try_clone()?;
        let client_output: TcpStream = client;
        let input = BufReader::new(client_input);
        let mut output = client_output;
        let mut lines = input.lines();

        // iteramos las lineas que recibimos de nuestro cliente
        while let Some(request) = lines.next() {
            //TODO: Wrappear esto a una func -> Result
            let (client_sndr, client_rcvr): (Sender<String>, Receiver<String>) = mpsc::channel();

            //TODO: Agregar decode
            let mut vector: Vec<String> = vec![];
            for string in request?.split_whitespace() {
                vector.push(string.to_string())
            }
            //TODO: FIN Agregar decode

            let command = generate(vector);
            let output_response;

            // TODO: Agregar forma de escritura por cada tipo.
            match command {
                Ok(command) => {
                    db_sender_clone
                        .send((command, client_sndr))
                        .map_err(|_| Error::new(ErrorKind::ConnectionAborted, "Db Sender error"))?;
                    let response = client_rcvr.recv().map_err(|_| {
                        Error::new(ErrorKind::ConnectionAborted, "Client receiver error")
                    })?;
                    output_response = response + "\n";
                }
                Err(msg) => {
                    output_response = msg + "\n";
                }
            };

            output.write_all(output_response.as_ref())?;
        }
        Ok(())

        //TODO: flag = false
    }

    fn db_thread(mut self, db_receiver: Receiver<(Command, Sender<String>)>) {
        let log_sender = self.log_sender.clone();
        let _: JoinHandle<Result<(), io::Error>> = thread::spawn(move || {
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

                if sender.send(output_response).is_err() {
                    log_sender
                        .send(Log::new(
                            LogLevel::Error,
                            line!(),
                            column!(),
                            file!().to_string(),
                            "DB sender error".to_string(),
                        ))
                        .map_err(|_| {
                            Error::new(ErrorKind::ConnectionAborted, "Log Sender error")
                        })?;
                }
            }
            Ok(())
        });
    }

    fn maintenance_thread(
        file: String,
        db_receiver: Sender<(Command, Sender<String>)>,
    ) -> io::Result<()> {
        loop {
            let (client_sndr, client_rcvr): (Sender<String>, Receiver<String>) = mpsc::channel();
            let command = Command::Store {
                path: file.to_string(),
            };

            db_receiver
                .send((command, client_sndr))
                .map_err(|_| Error::new(ErrorKind::ConnectionAborted, "DB receiver error"))?;
            client_rcvr
                .recv()
                .map_err(|_| Error::new(ErrorKind::ConnectionAborted, "DB sender error"))?;

            thread::sleep(Duration::from_secs(STORE_TIME_SEC));
        }
    }
}
