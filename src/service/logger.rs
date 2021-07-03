use std::sync::mpsc::Receiver;
use std::fs;
use std::io::Write;
use std::fs::OpenOptions;
use std::time::SystemTime;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Logger {
    receiver: Receiver<String>,
    path: String,
    level: u8,
}

impl Logger {
    #[allow(dead_code)]
    pub fn new(receiver: Receiver<String>, path: String) -> Self {
        Self { receiver, path, level: 1 }
    }

    pub fn log(self) {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(self.path).expect("Fail Open logfile");

        for element in self.receiver.recv() {
            let now = SystemTime::now();
            println!("{:?}", now);
            file.write((element + "\n").as_bytes());
        }
    }
}