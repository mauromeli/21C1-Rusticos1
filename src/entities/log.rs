use crate::entities::log_level::LogLevel;
use crate::service::timestamp_to_string::timestamp_to_string;
use std::time::SystemTime;

#[derive(Debug, Clone)]
#[allow(dead_code)]
/// Log: Struct usado para darle entidad a los datos necesarios para guardar un Log.
pub struct Log {
    level: LogLevel,
    line: u32,
    col: u32,
    file: String,
    msg: String,
}

impl Log {
    /// New: Constructor del struct de tipo Log.
    pub fn new(level: LogLevel, line: u32, col: u32, file: String, msg: String) -> Log {
        Self {
            level,
            line,
            col,
            file,
            msg,
        }
    }

    /// Retorna el Nivel de loggeo del Log
    pub fn get_level(self) -> u8 {
        match self.level {
            LogLevel::Error => 3,
            LogLevel::Info => 2,
            LogLevel::Debug => 1,
        }
    }
}

impl ToString for Log {
    /// Trait: impl usado para poder transformar un Log a tipo String.
    fn to_string(&self) -> String {
        let level = match self.level {
            LogLevel::Debug => "[DEBUG]",
            LogLevel::Info => "[INFO] ",
            LogLevel::Error => "[ERROR]",
        };

        level.to_owned()
            + " - "
            + &timestamp_to_string(SystemTime::now())
            + " UTC"
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

#[allow(unused_imports)]
mod test {
    use crate::entities::log::Log;
    use crate::entities::log_level::LogLevel;

    #[test]
    fn test_log_to_string() {
        let log = Log::new(
            LogLevel::Debug,
            10,
            10,
            "test".to_string(),
            "mensaje".to_string(),
        );
        assert_ne!("".to_string(), log.to_string());
    }

    #[allow(dead_code)]
    fn test_get_level() {
        let log = Log::new(
            LogLevel::Debug,
            10,
            10,
            "test".to_string(),
            "mensaje".to_string(),
        );
        assert_eq!(3, log.get_level());
        let log = Log::new(
            LogLevel::Info,
            10,
            10,
            "test".to_string(),
            "mensaje".to_string(),
        );
        assert_eq!(2, log.get_level());
        let log = Log::new(
            LogLevel::Error,
            10,
            10,
            "test".to_string(),
            "mensaje".to_string(),
        );
        assert_eq!(1, log.get_level());
    }
}
