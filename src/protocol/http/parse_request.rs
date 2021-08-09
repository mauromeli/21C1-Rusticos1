use std::collections::HashMap;

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub url: String,
    pub http_version: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

enum RequestParseState {
    Method,
    Url,
    HttpVersion,
    Headers { is_end: bool },
    Body,
}

pub enum HttpMethod {
    Get(String),
    Post(Vec<String>),
    Other(),
}

pub fn parse_command_rest(data: &[u8]) -> HttpMethod {
    let request = parse_request(data);
    match request.method.as_str() {
        "POST" => {
            let body = request.body;
            return if let Some(index_command) = body.find("command") {
                let command_len = 7;
                let equal = 1;
                let slice = &body[index_command + command_len + equal..];
                let command: Vec<String> = slice.split("+").map(String::from).collect();
                HttpMethod::Post(command)
            } else {
                HttpMethod::Post(vec![])
            };
        }
        "GET" => {
            let url = request.url;
            HttpMethod::Get(url)
        }
        _ => HttpMethod::Other(),
    }
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
                    } else if current == &b':'
                        && String::from_utf8(data[i + 1..i + 2].to_vec()).unwrap() == " "
                    {
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

    let method_slice = convert_to_string(&data[..=method]).unwrap();
    let url_slice = convert_to_string(&data[method + 2..=url]).unwrap();
    let http_version_slice = convert_to_string(&data[url + 2..=http_version]).unwrap();

    let mut headers = HashMap::new();
    let mut last = http_version + 3;

    for (key, value) in headers_key.iter().zip(headers_value) {
        let key_slice = convert_to_string(&data[last..*key + 1]).unwrap();
        let value_slice = convert_to_string(&data[key + 3..value + 1]).unwrap();
        last = value + 3;
        headers.insert(key_slice, value_slice);
    }

    let body_slice = convert_to_string(&data[body + 2..]).unwrap();

    Request {
        method: method_slice,
        url: url_slice,
        http_version: http_version_slice,
        headers,
        body: body_slice.trim_matches(char::from(0)).to_string(),
    }
}

fn convert_to_string(data: &[u8]) -> Result<String, String> {
    if let Ok(string) = String::from_utf8(data.to_vec()) {
        return Ok(string);
    }
    Err("Error intentando parsear el request".to_string())
}
