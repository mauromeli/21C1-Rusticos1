use std::io;

static END_LINES: &str = "<!--end-lines-->";
static INDEX_FILE: &str = "index.html";
static ERROR_FILE: &str = "404.html";

/// Html: estructura que representa el código HTML de nuestra página web.
pub struct Html {
    index: String,
}

impl Html {
    pub fn new() -> io::Result<Self> {
        let index = std::fs::read_to_string(INDEX_FILE)?;
        Ok(Self { index })
    }

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
            .replace(END_LINES, &(error_msg.to_string() + END_LINES));
    }

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
            .replace(END_LINES, &(input_msg.to_string() + END_LINES));
    }

    pub fn append_response(&mut self, msg: &str) {
        let response = format!(
            "<div class=\"line response\">\n
            <div class=\"nopad\">\n
            {}\n
            </div>\n
            </div>\n",
            msg
        );
        self.index = self.index.replace(END_LINES, &(response + END_LINES));
    }

    pub fn get_index(&self) -> String {
        self.index.clone()
    }

    pub fn get_resource(url: &str) -> io::Result<Vec<u8>> {
        std::fs::read(url)
    }

    pub fn get_404() -> io::Result<String> {
        std::fs::read_to_string(ERROR_FILE)
    }
}
