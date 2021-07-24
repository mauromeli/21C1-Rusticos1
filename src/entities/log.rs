use crate::entities::log_level::LogLevel;
use crate::service::timestamp_to_string::timestamp_to_string;
use std::time::SystemTime;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Log {
    level: LogLevel,
    line: u32,
    col: u32,
    file: String,
    msg: String,
}

impl Log {
    pub fn new(level: LogLevel, line: u32, col: u32, file: String, msg: String) -> Log {
        Self {
            level,
            line,
            col,
            file,
            msg,
        }
    }
}

impl ToString for Log {
    fn to_string(&self) -> String {
        let level = match self.level {
            LogLevel::Debug => "[DEBUG]",
            LogLevel::Info => "[INFO] ",
            LogLevel::Error => "[ERROR]",
        };

        timestamp_to_string(SystemTime::now())
            + " - "
            + level
            + " - "
            + &self.file
            + " - "
            + &self.line.to_string()
            + ":"
            + &self.col.to_string()
            + " - "
            + &self.msg
            + "\n"
    }
}
