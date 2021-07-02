use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::FromIterator;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct Config {
    verbose: u8,
    port: u16,
    timeout: u32,
    dbfilename: String,
    logfile: String,
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
            let tokens = Vec::from_iter(line.split_whitespace());
            let name = tokens.first().unwrap();
            let tokens = tokens.get(1..).unwrap();

            // Remuevo si hay un signo =
            let tokens = tokens.iter().filter(|t| !t.starts_with("="));
            // Remuevo si hay comentarios al final de los params
            let tokens = tokens.take_while(|t| !t.starts_with("#") && !t.starts_with(";"));

            // Concat back the parameters into one string to split for separated parameters
            let mut parameters = String::new();
            tokens.for_each(|t| {
                parameters.push_str(t);
                parameters.push(' ');
            });
            // Splits the parameters and trims
            let parameters = parameters.split(',').map(|s| s.trim());
            // Converts them from Vec<&str> into Vec<String>
            let parameters: Vec<String> = parameters.map(|s| s.to_string()).collect();

            let param = parameters[0].parse();
            if param.is_err() {
                continue;
            }
            // Setting the config parameters
            match name.to_lowercase().as_str() {
                "verbose" => config.set_verbose(param.unwrap()),
                "port" => config.set_port(param.unwrap() as u16),
                "timeout" => config.set_timeout(param.unwrap() as u32),
                "dbfilename" => config.set_dbfilename(param.unwrap().to_string()),
                "logfile" => config.set_logfile(param.unwrap().to_string()),
                _ => (),
            }
        }

        config
    }

    fn set_verbose(&mut self, verbose: u8) {
        self.verbose = verbose;
    }

    fn set_port(&mut self, port: u16) {
        self.port = port;
    }

    fn set_timeout(&mut self, timeout: u32) {
        self.timeout = timeout;
    }

    fn set_dbfilename(&mut self, dbfilename: String) {
        self.dbfilename = dbfilename;
    }

    fn set_logfile(&mut self, logfile: String) {
        self.logfile = logfile;
    }

    pub fn get_port(&self) -> String {
        self.port.to_string()
    }
}

fn is_invalid_line(line: &str) -> bool {
    line.starts_with("#") || line.starts_with(";") || line.is_empty()
}
