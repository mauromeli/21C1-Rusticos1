use std::collections::HashMap;

#[derive(Debug)]
pub struct Request<'a> {
    pub method: String,
    pub url: String,
    pub http_version: String,
    pub headers: HashMap<&'a [u8], &'a [u8]>,
    pub body: String,
}

enum RequestParseState {
    Method,
    Url,
    HttpVersion,
    Headers { is_end: bool },
    Body,
}

pub fn parse_command_rest(data: &[u8]) -> Vec<String> {
    let request = parse_request(data);
    if request.method == "POST" {
        let url = request.url;
        if let Some(index_command) = url.find("command") {
            let command_len = 7;
            let equal = 1;
            let slice = &url[index_command + command_len + equal..];
            let command = slice.split("%20").map(String::from).collect();
            return command;
        } else {
            return vec![];
        }
    }
    return vec![];
}

fn parse_request(data: &[u8]) -> Request {
    let mut state = RequestParseState::Method;
    let mut method = 0;
    let mut url = 0;
    let mut http_version = 0;
    let mut header = 0;
    let mut body = 0;
    let mut headers_key: Vec<usize> = vec![];
    let mut headers_value: Vec<usize> = vec![];
    for (i, current) in data.iter().enumerate() {
        match state {
            RequestParseState::Method => {
                if current == &b' ' {
                    state = RequestParseState::Url;
                } else {
                    method = i;
                }
            }
            RequestParseState::Url => {
                if current == &b' ' {
                    state = RequestParseState::HttpVersion;
                } else {
                    url = i;
                }
            }
            RequestParseState::HttpVersion => {
                if current == &b'\n' {
                    state = RequestParseState::Headers { is_end: false };
                } else {
                    if current != &b'\r' {
                        http_version = i;
                    }
                }
            }
            RequestParseState::Headers { is_end } => {
                if is_end {
                    if current == &b'\n' {
                        state = RequestParseState::Body;
                    }
                } else {
                    if current == &b'\r' {
                        if String::from_utf8(data[header + 3..header + 4].to_vec()).unwrap() == "\r"
                        {
                            state = RequestParseState::Headers { is_end: true };
                        } else {
                            headers_value.push(header);
                            header = 0;
                        }
                    } else if current == &b':' {
                        headers_key.push(header);
                        header = 0;
                    } else {
                        header = i;
                    }
                }
            }
            RequestParseState::Body => {
                body = i;
                break;
            }
        }
    }

    let method_slice = &data[..=method];
    let url_slice = &data[method + 2..=url];
    let http_version_slice = &data[url + 2..=http_version];

    let mut headers = HashMap::new();
    let mut last = http_version + 2;
    /*
    falta parte de los headers
        for (key, value) in headers_key.iter().zip(headers_value) {
        let key_slice = &data[last..*key];
        let value_slice = &data[key + 2..value];
        last = value + 2;
        headers.insert(key_slice, value_slice);
    }
     */
    let body_slice = &data[body + 2..];

    Request {
        method: String::from_utf8(method_slice.to_vec()).unwrap(),
        url: String::from_utf8(url_slice.to_vec()).unwrap(),
        http_version: String::from_utf8(http_version_slice.to_vec()).unwrap(),
        headers: headers,
        body: String::from_utf8(body_slice.to_vec()).unwrap(),
    }
}
