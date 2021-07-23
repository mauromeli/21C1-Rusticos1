use crate::entities::log::Log;
use std::fmt::Debug;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::mpsc::Receiver;
use std::thread;

#[derive(Debug)]
pub struct Logger {
    receiver: Receiver<Log>,
    path: String,
    verbose: u8,
}

impl Logger {
    #[allow(dead_code)]
    pub fn new(receiver: Receiver<Log>, path: String) -> Self {
        Self {
            receiver,
            path,
            verbose: 1,
        }
    }

    #[allow(unused_must_use)]
    pub fn log(self) {
        let _: std::thread::JoinHandle<Result<(), std::io::Error>> = thread::spawn(move || {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(self.path.clone())?;

            while let Ok(log) = self.receiver.recv() {
                /*if path != path_new => {
                                let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(self.path)
                }*/

                if self.verbose == 1 {
                    println!("{:?}", log.to_string());
                }

                file.write(log.to_string().as_bytes());
            }
            Ok(())
        });
    }
}
