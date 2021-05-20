#[derive(Debug)]
struct Redis {
    db: HashMap<String, String>,
}

impl Redis {
    pub fn new() -> Self {
        let mut map: HashMap<String, String> = HashMap::new();
        Self {
            db: map
        }
    }

    pub fn execute(mut self, key: String, value: String) -> Result<String, String> {

        match key {
            "ping" => return "pong",
            "get" => db.get(key),
            "set" => db.insert(key, value),
            _ => return Err("Command not valid"),
        }
    }
}