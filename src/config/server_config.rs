use crate::entities::log_level::LogLevel;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

// Struct usado para representar la configuración posible de nuestra base de datos Redis.

#[derive(Debug)]
pub struct Config {
    verbose: u8,
    port: u16,
    timeout: u64,
    dbfilename: String,
    logfile: String,
    loglevel: LogLevel,
}

#[allow(dead_code)]
impl Config {
    pub fn new() -> Config {
        Config {
            verbose: 0,
            port: 6379,
            timeout: 0,
            dbfilename: "dump.rdb".to_string(),
            logfile: "log.log".to_string(),
            loglevel: LogLevel::Debug,
        }
    }

    pub fn new_from_file(path: String) -> Config {
        let path = Path::new(&path);
        let file = File::open(path).expect("File not found or cannot be opened");
        let content = BufReader::new(&file);
        let mut config = Config::new();

        for line in content.lines() {
            let line = line.expect("Could not read the line");
            // Remuevo espacios al principio y al final de la línea.
            let line = line.trim();

            // Verifíco si la línea es valida; eg: comentarios, linea en blanco, etc.
            if is_invalid_line(line) {
                continue;
            }

            // Separo el attributo de la config con el valor de la config.
            let splited: Vec<&str> = line.split_whitespace().collect();
            let name = splited.first().unwrap();
            let tokens = splited.get(1..).unwrap();

            let parameters = Config::clean_and_parse_lines(tokens);
            let param = parameters[0].clone();

            // Setting the config parameters
            match name.to_lowercase().as_str() {
                "verbose" => config.set_verbose(param),
                "port" => config.set_port(param),
                "timeout" => config.set_timeout(param),
                "dbfilename" => config.set_dbfilename(param),
                "logfile" => config.set_logfile(param),
                "loglevel" => config.set_loglevel(param),
                _ => (),
            }
        }

        config
    }

    fn clean_and_parse_lines(tokens: &[&str]) -> Vec<String> {
        // Remuevo si hay un signo =
        let tokens = tokens.iter().filter(|t| !t.starts_with('='));
        // Remuevo si hay comentarios al final de los params
        let tokens = tokens.take_while(|t| !t.starts_with('#') && !t.starts_with(';'));

        // Concat back the parameters into one string to split for separated parameters
        let mut parameters = String::new();
        tokens.for_each(|t| {
            parameters.push_str(t);
            parameters.push(' ');
        });
        // Splits the parameters and trims
        let parameters = parameters.split(',').map(|s| s.trim());
        // Converts them from Vec<&str> into Vec<String>
        let parameters: Vec<String> = parameters.map(|s| s.to_stri>>>>>>> monitortomodifyng()).collect();
        parameters
    }

    fn set_verbose(&mut self, verbose: String) {
        let val = verbose.parse::<u8>();
        if let Ok(value) = val {
            self.verbose = value
        }
    }

    fn set_port(&mut self, port: String) {
        let val = port.parse::<u16>();
        if let Ok(value) = val {
            self.port = value
        }
    }

    fn set_timeout(&mut self, timeout: String) {
        let val = timeout.parse::<u64>();
        if let Ok(value) = val {
            self.timeout = value
        }
    }

    fn set_dbfilename(&mut self, dbfilename: String) {
        self.dbfilename = dbfilename;
    }

    fn set_logfile(&mut self, logfile: String) {
        self.logfile = logfile;
    }

    fn set_loglevel(&mut self, loglevel: String) {
        match loglevel.to_lowercase().as_str() {
            "error" => self.loglevel = LogLevel::Error,
            "info" => self.loglevel = LogLevel::Info,
            _ => self.loglevel = LogLevel::Debug,
        }
    }

    pub fn get_port(&self) -> String {
        self.port.to_string()
    }

    pub fn get_verbose(&self) -> String {
        self.verbose.to_string()
    }

    pub fn get_timeout(&self) -> u64 {
        self.timeout
    }

    pub fn get_dbfilename(&self) -> String {
        self.dbfilename.to_string()
    }

    pub fn get_logfile(&self) -> String {
        self.logfile.to_string()
    }
}

fn is_invalid_line(line: &str) -> bool {
    line.starts_with('#') || line.starts_with(';') || line.is_empty()
}

#[allow(unused_imports)]
mod test {
    use crate::config::server_config::{is_invalid_line, Config};
    use crate::entities::log_level::LogLevel;
    use std::iter::FromIterator;

    #[test]
    fn check_default_config_values() {
        let config = Config::new();
        assert_eq!("0", config.get_verbose());
        assert_eq!("6379", config.get_port());
        assert_eq!(0, config.get_timeout());
        assert_eq!("dump.rdb".to_string(), config.get_dbfilename());
        assert_eq!("log.log".to_string(), config.get_logfile());
        assert_eq!(LogLevel::Debug, config.loglevel);
    }

    #[test]
    fn clean_and_parse_lines() {
        let line: &str = "dbnombre.rbd # Listado de elementos comentados";
        let splited = Vec::from_iter(line.split_whitespace());
        let vec = splited.get(0..).unwrap();
        let params = Config::clean_and_parse_lines(vec);

        assert_eq!(1, params.len())
    }

    #[test]
    fn check_line_is_valid_false() {
        let line: &str = "#esta línea no es valida";
        assert!(is_invalid_line(line));

        let line: &str = ";esta línea no es valida";
        assert!(is_invalid_line(line));

        let line: &str = "esta línea es valida";
        assert!(!is_invalid_line(line))
    }
}
