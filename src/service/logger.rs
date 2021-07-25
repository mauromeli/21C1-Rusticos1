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
pub struct Logger {
    receiver: Receiver<Log>,
    verbose: u8,
    config: Arc<Mutex<Config>>,
}

impl Logger {
    #[allow(dead_code)]
    pub fn new(receiver: Receiver<Log>, config: Arc<Mutex<Config>>) -> Self {
        Self {
            receiver,
            verbose: 1,
            config,
        }
    }

    #[allow(unused_must_use)]
    pub fn log(self) {
        let _: JoinHandle<Result<(), Error>> = thread::spawn(move || {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(self.config.lock().unwrap().get_logfile())?;

            while let Ok(log) = self.receiver.recv() {
                if self.verbose == 1 {
                    println!("{:?}", log.to_string());
                }

                file.write(log.to_string().as_bytes());
            }
            Ok(())
        });
    }
}
