use crate::config::server_config::Config;
use crate::entities::command::Command;
use crate::entities::log::Log;
use crate::entities::log_level::LogLevel;
use crate::entities::response::Response;
use crate::protocol::decode::{decode, TypeData};
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

use crate::protocol::parse_data::{parse_command, parse_response_error, parse_response_ok};
use std::thread::JoinHandle;
use std::time::Duration;

/// Tiempo de ejecución entre un ciclo y el siguiente, en el hilo de Mantenimiento.
/// Este valor está representado en Segundos.
static STORE_TIME_SEC: u64 = 120;

/// Tipo de dato definido para guardar las conecciones de los usuarios y su estado en uso.
type VecHandler = Vec<(JoinHandle<Result<(), io::Error>>, Arc<AtomicBool>)>;
/// Tipo de dato definido para el canal de envío de mensajes al hilo ejecutor de comandos en DB
type DbSender = Sender<(Command, Sender<Response>)>;
/// Tipo de dato definido para el canal de envío de mensajes al hilo ejecutor de comandos en DB
type DbReceiver = Receiver<(Command, Sender<Response>)>;

#[derive(Debug)]
/// Struct utilizado para representar la entidad Server dentro del Modelo.
/// Este server atenderá:
/// - Las solicitudes de los clientes paa conectarse.
/// - Se comunicará con la Base de datos Redis
pub struct Server {
    /// Instancia de la Base de Datos
    redis: Redis,
    /// Canal para enviar eventos de loggeo al Logger
    log_sender: Sender<Log>,
    /// Configuración del servidor compartida.
    config: Arc<Mutex<Config>>,
}

impl Server {
    #[allow(dead_code)]
    /// Constructor del Servidor. Para Construir el mismo se necesita una instancia de tipo Config.
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

    /// Methodo del Server para ponerlo operativo.
    pub fn serve(mut self) -> Result<(), Box<dyn std::error::Error>> {
        // load db
        let command = Command::Load {
            path: self.config.lock().unwrap().get_dbfilename(),
        };
        self.redis.execute(command)?;

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
        let (db_sender, db_receiver): (DbSender, DbReceiver) = mpsc::channel();

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

        let mut handlers: VecHandler = vec![];

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

            let flag = Arc::new(AtomicBool::new(true));
            let used_flag = flag.clone();
            let handler: JoinHandle<Result<(), io::Error>> = thread::spawn(move || {
                Server::client_handler(client, db_sender_clone, &used_flag)?;
                Ok(())
            });
            handlers.push((handler, flag));

            let mut handlers_actives: VecHandler = vec![];
            let mut handlers_inactives: VecHandler = vec![];
            for (handler, used) in handlers {
                if used.load(Ordering::Relaxed) {
                    handlers_actives.push((handler, used));
                } else {
                    handlers_inactives.push((handler, used));
                }
            }

            for (handler, _) in handlers_inactives {
                if handler.join().is_err() {
                    log_sender
                        .send(Log::new(
                            LogLevel::Error,
                            line!(),
                            column!(),
                            file!().to_string(),
                            "Error joining handler".to_string(),
                        ))
                        .map_err(|_| {
                            Error::new(ErrorKind::ConnectionAborted, "Error joining handler")
                        })?;
                }
            }

            handlers = handlers_actives;
        }

        Ok(())
    }

    #[allow(clippy::while_let_on_iterator)]
    /// Metodo encargado de capturar los eventos de cada cliente.
    fn client_handler(
        client: TcpStream,
        db_sender_clone: Sender<(Command, Sender<Response>)>,
        used: &AtomicBool,
    ) -> io::Result<()> {
        let client_input: TcpStream = client.try_clone()?;
        let client_output: TcpStream = client;
        let mut input = BufReader::new(client_input);
        let mut output = client_output;

        let client_id = output.try_clone()?.local_addr()?.to_string();

        Server::connected_user(&db_sender_clone);

        // iteramos las lineas que recibimos de nuestro cliente
        'principal: while let Some(line) = LinesIterator::new(&mut input).next() {
            //TODO: Wrappear esto a una func -> Result
            let (client_sndr, client_rcvr): (Sender<Response>, Receiver<Response>) =
                mpsc::channel();

            let vector = parse_command(line);

            let command = generate(vector, client_id.clone());

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
                            output.write_all(&parse_response_ok(redis_string))?;
                        }
                        Response::Stream(rec) => {
                            'inner: while let Ok(redis_element) = rec.recv() {
                                if output.write_all(&parse_response_ok(redis_element)).is_err() {
                                    break 'inner;
                                }
                            }

                            std::mem::drop(rec);
                            break 'principal;
                        }
                        Response::Error(msg) => {
                            output.write_all(&parse_response_error(msg))?;
                        }
                    }
                }
                Err(err) => {
                    output.write_all(&parse_response_error(err))?;
                }
            };
        }

        used.swap(false, Ordering::Relaxed);
        Server::disconnected_user(&db_sender_clone);

        Ok(())
    }

    /// Metodo encargado de Enviarle una señal a la DB indicando que se ha conectado otro usuario.
    fn connected_user(db_sender_clone: &Sender<(Command, Sender<Response>)>) {
        let (client_sndr, client_rcvr): (Sender<Response>, Receiver<Response>) = mpsc::channel();
        let _ = db_sender_clone.send((Command::AddClient, client_sndr));
        let _ = client_rcvr.recv();
    }

    /// Metodo encargado de Enviarle una señal a la DB indicando que se ha desconectado un usuario.
    fn disconnected_user(db_sender_clone: &Sender<(Command, Sender<Response>)>) {
        let (client_sndr, client_rcvr): (Sender<Response>, Receiver<Response>) = mpsc::channel();
        let _ = db_sender_clone.send((Command::RemoveClient, client_sndr));
        let _ = client_rcvr.recv();
    }

    /// Metodo encargado de centralizar las ejecuciones de los comandos que se ejecutan en la DB.
    /// El servidor le envía un canal de Recepción de Comandos y Senders donde debe enviar la
    /// respuesta al cliente.
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

    /// Metodo ejecutado en el hilo de mantenimiento el cual se encarga de ejecutar acciones dentro
    /// del server que sean de Mantenimiento. Como por ejemplo persistir la base de datos en caso de
    /// fallas.
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

pub struct LinesIterator<'a> {
    input: &'a mut BufReader<TcpStream>,
}

impl<'a> LinesIterator<'a> {
    pub fn new(input: &'a mut BufReader<TcpStream>) -> Self {
        let input = input;
        Self { input }
    }
}

impl Iterator for LinesIterator<'_> {
    type Item = TypeData;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let mut buf = String::new();
        while self.input.read_line(&mut buf).ok()? != 0 {
            if let Ok(result) = decode(buf.as_bytes(), 0) {
                let (data, _) = result;
                return Some(data);
            }
        }
        Some(TypeData::Error("Se ha producido un error".to_string()))
    }
}
