use crate::config::server_config::Config;
use crate::entities::command::Command;
use crate::entities::log::Log;
use crate::entities::log_level::LogLevel;
use crate::entities::response::Response;
use crate::service::command_generator::generate;
use crate::service::logger::Logger;
use crate::service::redis::Redis;
use std::io;
use std::io::{BufRead, BufReader, Error, ErrorKind, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
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
        let (db_sender, db_receiver): (
            Sender<(Command, Sender<Response>)>,
            Receiver<(Command, Sender<Response>)>,
        ) = mpsc::channel();

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

        let mut handlers: Vec<(JoinHandle<Result<(), io::Error>>, Arc<AtomicBool>)> = vec![];

        while let Ok(connection) = listener.accept() {
            //accepter thread
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
            let db_sender_clone: Sender<(Command, Sender<Response>)> = db_sender.clone();
            //TODO: Handler client. encolar en vector booleano compartido para finalizar hilos.

            let flag = Arc::new(AtomicBool::new(true));
            let used_flag = flag.clone();
            let handler: JoinHandle<Result<(), io::Error>> = thread::spawn(move || {
                Server::client_handler(client, db_sender_clone, &used_flag)?;
                Ok(())
            });
            handlers.push((handler, flag));

            let mut handlers_actives: Vec<(JoinHandle<Result<(), io::Error>>, Arc<AtomicBool>)> =
                vec![];
            let mut handlers_inactives: Vec<(JoinHandle<Result<(), io::Error>>, Arc<AtomicBool>)> =
                vec![];
            for (handler, used) in handlers {
                if used.load(Ordering::Relaxed) {
                    handlers_actives.push((handler, used));
                } else {
                    handlers_inactives.push((handler, used));
                }
            }

            for (handler, _) in handlers_inactives {
                // TODO: revisar salida
                let _ = handler.join();
            }

            handlers = handlers_actives;
        }

        Ok(())
    }

    #[allow(clippy::while_let_on_iterator)]
    fn client_handler(
        client: TcpStream,
        db_sender_clone: Sender<(Command, Sender<Response>)>,
        used: &AtomicBool,
    ) -> io::Result<()> {
        let client_input: TcpStream = client.try_clone()?;
        let client_output: TcpStream = client;
        let input = BufReader::new(client_input);
        let mut output = client_output;
        let mut lines = input.lines();

        //TODO: ver error
        Server::connected_user(&db_sender_clone);

        // iteramos las lineas que recibimos de nuestro cliente
        while let Some(request) = lines.next() {
            //TODO: Wrappear esto a una func -> Result
            let (client_sndr, client_rcvr): (Sender<Response>, Receiver<Response>) =
                mpsc::channel();

            //TODO: Agregar decode
            let mut vector: Vec<String> = vec![];
            for string in request?.split_whitespace() {
                vector.push(string.to_string())
            }
            //TODO: FIN Agregar decode

            let command = generate(vector);

            // TODO: Agregar forma de escritura por cada tipo.
            match command {
                Ok(command) => {
                    db_sender_clone
                        .send((command, client_sndr))
                        .map_err(|_| Error::new(ErrorKind::ConnectionAborted, "Db Sender error"))?;

                    let response = client_rcvr.recv().map_err(|_| {
                        Error::new(ErrorKind::ConnectionAborted, "Client receiver error")
                    })?;

                    match response {
                        Response::Normal(redis_string) => {
                            output.write((redis_string.to_string() + "\n").as_ref())?;
                        }
                        Response::Stream(rec) => {
                            while let Ok(redis_element) = rec.recv() {
                                output.write((redis_element.to_string() + "\n").as_ref())?;
                                println!("msg");
                            }
                            println!("SALIO");
                            std::mem::drop(rec);
                        }
                        Response::Error(msg) => {
                            output.write((msg + "\n").as_ref())?;
                        }
                    }
                }
                _ => {
                    output.write((command.err().unwrap() + "\n").as_ref())?;
                }
            };
        }

        used.swap(false, Ordering::Relaxed);
        Server::disconnected_user(&db_sender_clone);

        Ok(())
    }

    fn connected_user(db_sender_clone: &Sender<(Command, Sender<Response>)>) {
        let (client_sndr, client_rcvr): (Sender<Response>, Receiver<Response>) = mpsc::channel();
        let _ = db_sender_clone.send((Command::AddClient, client_sndr));
        let _ = client_rcvr.recv();
    }

    fn disconnected_user(db_sender_clone: &Sender<(Command, Sender<Response>)>) {
        let (client_sndr, client_rcvr): (Sender<Response>, Receiver<Response>) = mpsc::channel();
        let _ = db_sender_clone.send((Command::RemoveClient, client_sndr));
        let _ = client_rcvr.recv();
    }

    fn db_thread(mut self, db_receiver: Receiver<(Command, Sender<Response>)>) {
        let log_sender = self.log_sender.clone();
        let _: JoinHandle<Result<(), io::Error>> = thread::spawn(move || {
            while let Ok((command, sender)) = db_receiver.recv() {
                let redis_response = self.redis.execute(command);
                match redis_response {
                    Ok(value) => {
                        if sender.send(value).is_err() {
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
                    Err(error_msg) => {
                        if sender.send(Response::Error(error_msg)).is_err() {
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
                };
            }
            Ok(())
        });
    }

    fn maintenance_thread(
        file: String,
        db_receiver: Sender<(Command, Sender<Response>)>,
    ) -> io::Result<()> {
        loop {
            let (client_sndr, client_rcvr): (Sender<Response>, Receiver<Response>) =
                mpsc::channel();
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
