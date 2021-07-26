use crate::config::server_config::Config;
use crate::entities::log::Log;
use std::fmt::Debug;
use std::fs::OpenOptions;
use std::io::{Error, Write};
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

#[derive(Debug)]
/// Entidad para loggear los eventos que ocurren en el servidor redis.
pub struct Logger {
    /// Canal donde el logger recibirá los logs a loggear.
    receiver: Receiver<Log>,
    /// Indica si los mensajes de log se imprimen por consola.
    verbose: u8,
    /// Configuración del servidor compartida.
    config: Arc<Mutex<Config>>,
    /// Nivel de loggeo que fue seteado
    loglevel: u8,
    file: String,
}

impl Logger {
    #[allow(dead_code)]
    /// Constructor de un nuevo Logger
    pub fn new(receiver: Receiver<Log>, config: Arc<Mutex<Config>>, level: u8) -> Self {
        let file = config.lock().unwrap().get_logfile();
        Self {
            receiver,
            verbose: 1,
            config,
            loglevel: level,
            file,
        }
    }

    #[allow(unused_must_use)]
    /// Servicio de loggeo
    pub fn log(mut self) {
        let _: JoinHandle<Result<(), Error>> = thread::spawn(move || {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(self.config.lock().unwrap().get_logfile())?;

            while let Ok(log) = self.receiver.recv() {
                if self.file != self.config.lock().unwrap().get_logfile() {
                    file = OpenOptions::new()
                        .write(true)
                        .create(true)
                        .append(true)
                        .open(self.config.lock().unwrap().get_logfile())?;
                    self.file = self.config.lock().unwrap().get_logfile();
                }

                if self.verbose == 1 {
                    println!("{:?}", log.clone().to_string());
                }

                let level = log.clone().get_level();
                if level <= self.loglevel {
                    file.write(log.to_string().as_bytes());
                }
            }
            Ok(())
        });
    }
}
