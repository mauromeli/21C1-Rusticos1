use crate::config::server_config::Config;
use crate::entities::command::Command;
use crate::entities::log::Log;
use crate::entities::log_level::LogLevel;
use crate::entities::response::Response;
use crate::service::command_generator::generate;
use crate::service::logger::Logger;
use crate::service::redis::Redis;
use std::io::{BufRead, BufReader, Write, Lines, Read, Error};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc};
use std::thread;
use crate::protocol::parse_data::parse_data;
use std::borrow::BorrowMut;
use crate::protocol::decode::{decode, TypeData};

use std::thread::JoinHandle;
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
        let (db_sender, db_receiver): (
            Sender<(Command, Sender<Response>)>,
            Receiver<(Command, Sender<Response>)>,
        ) = mpsc::channel();

        let log_sender = self.log_sender.clone();
        let timeout = self.config.get_timeout();

        let db_filename = self.config.get_dbfilename();
        let db_sender_maintenance = db_sender.clone();

        //Todo: Agregar el handler.
        let _ =
            thread::spawn(move || Server::maintenance_thread(db_filename, db_sender_maintenance));

        self.db_thread(db_receiver);

        let mut handlers: Vec<(JoinHandle<()>, Arc<AtomicBool>)> = vec![];

        while let Ok(connection) = listener.accept() {
            //accepter thread
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
            let db_sender_clone: Sender<(Command, Sender<Response>)> = db_sender.clone();
            //TODO: Handler client. encolar en vector booleano compartido para finalizar hilos.
            //let used = Arc::clone(&shared);
            let flag = Arc::new(AtomicBool::new(true));
            let used_flag = flag.clone();
            let handler =
                thread::spawn(move || Server::client_handler(client, db_sender_clone, &used_flag));
            handlers.push((handler, flag));
            println!("handlers {:?}", handlers);
            //antes de hacer el join me quedo con los true y luego los false para hacerle join.
            // if vive lo guardo else join.
            //let vec = handlers.iter().filter(|h| h.1 == false).map().collect();
            /*for handler in handlers.filter(used==false).iter() {
                handler.join()
            }*/
            //let mut handlers_actives: Vec<(JoinHandle<()>, Arc<AtomicBool>)> = vec![];
            //let handlers_actives: Vec<(JoinHandle<()>, Arc<AtomicBool>)>= handlers.iter().filter(|&x| x.1.load(Ordering::Relaxed)).collect();
            /*for x in &handlers {
                if x.1.load(Ordering::Relaxed) {
                    handlers_actives.push(x);
                } else {
                    //TODO lanzar este result
                    let result = x.0.join();
                }
            }*/
            println!("index {:?}", handlers);
        }
    }

    #[allow(clippy::while_let_on_iterator)]
    fn client_handler(
        client: TcpStream,
        db_sender_clone: Sender<(Command, Sender<Response>)>,
        used: &AtomicBool,
    ) {
        let client_input: TcpStream = client.try_clone().unwrap();
        let client_output: TcpStream = client;
        let mut input = BufReader::new(client_input);
        let mut output = client_output;
        //let mut lines = input.lines();

        //TODO: ver error
        Server::connected_user(&db_sender_clone);


        while let Some(line) = LinesIterator::new(&mut input).next() {
            //TODO: Wrappear esto a una func -> Result
            let (client_sndr, client_rcvr): (Sender<Response>, Receiver<Response>) =
                mpsc::channel();

            let vector = parse_data(line);

            //TODO: FIN Agregar decode

            let command = generate(vector);

            // TODO: Agregar forma de escritura por cada tipo.
            match command {
                /*
                Ok(Command::Command) => {
                    //output_response = "*200\r\n".to_string();
                }

                 */

                Ok(command) => {
                    let _ = db_sender_clone.send((command, client_sndr));
                    let response = client_rcvr.recv().unwrap();

                    match response {
                        Response::Normal(redis_string) => {
                            let _ = output.write((redis_string.to_string() + "\n").as_ref());
                        }
                        Response::Stream(rec) => {
                            while let Ok(redis_element) = rec.recv() {
                                let _ = output.write((redis_element.to_string() + "\n").as_ref());
                            }
                        }
                        Response::Error(msg) => {
                            let _ = output.write((msg + "\n").as_ref());
                        }
                    }
                }
                _ => {
                    let _ = output.write((command.err().unwrap() + "\n").as_ref());
                }
            };
            //TODO: Ver que hacer con el error.
        }

        //TODO: flag = false
        used.swap(false, Ordering::Relaxed);
        Server::disconnected_user(&db_sender_clone);
    }

    fn connected_user(db_sender_clone: &Sender<(Command, Sender<Response>)>) {
        let (client_sndr, client_rcvr): (Sender<Response>, Receiver<Response>) =
            mpsc::channel();
        let _ = db_sender_clone.send((Command::AddClient, client_sndr));
        let _ = client_rcvr.recv();
    }

    fn disconnected_user(db_sender_clone: &Sender<(Command, Sender<Response>)>) {
        let (client_sndr, client_rcvr): (Sender<Response>, Receiver<Response>) =
            mpsc::channel();
        let _ = db_sender_clone.send((Command::RemoveClient, client_sndr));
        let _ = client_rcvr.recv();
    }

    fn db_thread(mut self, db_receiver: Receiver<(Command, Sender<Response>)>) {
        let _ = thread::spawn(move || {
            while let Ok((command, sender)) = db_receiver.recv() {
                let redis_response = self.redis.execute(command);
                match redis_response {
                    Ok(value) => {
                        let _ = sender.send(value);
                    }
                    Err(error_msg) => {
                        let _ = sender.send(Response::Error(error_msg));
                    }
                };
            }
        });
    }
}

    //TODO: Return Result. -> Result<(), std::io::Error>
    fn maintenance_thread(file: String, db_receiver: Sender<(Command, Sender<Response>)>) {
        loop {
            let (client_sndr, client_rcvr): (Sender<Response>, Receiver<Response>) =
                mpsc::channel();
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
    }

