use std::io;

/// Representa `start-lines` de nuestro HTML.
static START_LINES: &str = "<!--start-lines-->";
/// Representa el nombre de nuestro archivo HTML.
static INDEX_FILE: &str = "index.html";
/// Representa el nombre de nuestro archivo HTML en caso de error 404.
static ERROR_FILE: &str = "404.html";

/// Estructura que representa el código HTML de nuestra página web.
pub struct Html {
    ///Representa el archivo en donde estará el codigo HTML.
    index: String,
}

impl Html {
    pub fn new() -> io::Result<Self> {
        let index = std::fs::read_to_string(INDEX_FILE)?;
        Ok(Self { index })
    }

    /// Agrega el código HTML en nuestro archivo, en el caso de que se deba mostrar un error.
    ///
    /// # Arguments
    ///
    /// * `msg` - Representa el mensaje de error.
    pub fn append_error(&mut self, msg: &str) {
        let error_msg = format!(
            "<div class=\"line error\">\n
            <div class=\"nopad\">\n
            (error) {}\n
            </div>\n
            </div>\n",
            msg
        );
        self.index = self
            .index
            .replace(START_LINES, &(START_LINES.to_owned() + &error_msg));
    }

    /// Agrega el código HTML en nuestro archivo, en el caso de que se deba mostrar un input.
    ///
    /// # Arguments
    ///
    /// * `input` - Representa el input.
    pub fn append_input(&mut self, input: &str) {
        let input_msg = format!(
            "<div class=\"line input\">\n
            <div class=\"nopad\">\n
            <span class=\"prompt\">\n
            &gt;
            </span>\n
            {}\n
            </div>\n
            </div>\n",
            input
        );
        self.index = self
            .index
            .replace(START_LINES, &(START_LINES.to_owned() + &input_msg));
    }

    /// Agrega el código HTML en nuestro archivo, en el caso de que se deba mostrar una respuesta.
    ///
    /// # Arguments
    ///
    /// * `msg` - Representa el mensaje de respuesta.
    pub fn append_response(&mut self, msg: &str) {
        let response = format!(
            "<div class=\"line response\">\n
            <div class=\"nopad\">\n
            {}\n
            </div>\n
            </div>\n",
            msg
        );
        self.index = self
            .index
            .replace(START_LINES, &(START_LINES.to_owned() + &response));
    }

    /// Devuelve el código HTML.
    pub fn get_index(&self) -> String {
        self.index.clone()
    }

    /// Devuelve los bytes de las fotos a cargar en nuestro HTML.
    pub fn get_resource(url: &str) -> io::Result<Vec<u8>> {
        std::fs::read(url)
    }

    /// Devuelve el código HTML en caso de error 404.
    pub fn get_404() -> io::Result<String> {
        std::fs::read_to_string(ERROR_FILE)
    }
}
