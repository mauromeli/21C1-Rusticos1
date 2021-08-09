/// Html: estructura que representa el código HTML de nuestra página web.
pub struct Html {
    index: String,
}

impl Html {
    pub fn new() -> io::Result<Self> {
        let index = std::fs::read_to_string(HTML_FILE)?;
        Ok(Self { index })
    }

    pub fn append_error(&mut self, msg: &str) {
        let error_msg = format!(
            "\t<div class=\"line error\">
            \t\t\n<div class=\"nopad\">
            \n\t\t(error) {}
            \n\t\t</div>
            \n\t</div>\n",
            msg
        );
        self.index = self
            .index
            .replace(HTML_LINES, &(error_msg.to_string() + HTML_LINES));
    }

    pub fn append_input(&mut self, input: &str) {
        let input_msg = format!(
            "\t<div class=\"line input\">
            \n\t\t<span class=\"prompt\">
            \n\t\t&gt;
            \n\t\t</span>
            \n\t\t<div class=\"nopad\">
            \n\t\t{}
            \n\t\t</div>
            \n\t</div>\n",
            input
        );
        self.index = self
            .index
            .replace(HTML_LINES, &(input_msg.to_string() + HTML_LINES));
    }

    pub fn append_response(&mut self, msg: &str) {
        let response = format!(
            "\t<div class=\"line response\">
            \n\t\t<div class=\"nopad\">
            \n\t\t{}
            \n\t\t</div>
            \n\t</div>\n",
            msg
        );
        self.index = self.index.replace(HTML_LINES, &(response + HTML_LINES));
    }

    pub fn get_index(&self) -> String {
        self.index.clone()
    }

    pub fn get_len(&self) -> usize {
        self.index.len()
    }
}