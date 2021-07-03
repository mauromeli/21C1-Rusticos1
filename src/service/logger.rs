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
    level: u8,
}

impl Logger {
    #[allow(dead_code)]
    pub fn new(receiver: Receiver<Log>, path: String) -> Self {
        Self {
            receiver,
            path,
            level: 1,
        }
    }

    #[allow(unused_must_use)]
    pub fn log(self) {
        let _ = thread::spawn(move || {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(self.path)
                .expect("Fail Open logfile");

            while let Ok(log) = self.receiver.recv() {
                file.write(log.to_string().as_bytes());
            }
        });
    }
}
