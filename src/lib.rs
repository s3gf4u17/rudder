use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpListener;

pub struct Request {}

pub struct Response {
    status: String,
    datatype: String,
    length: usize,
    contents: String,
}

pub struct Router {
    host: String,
    port: String,
    router: HashMap<String, fn(Request) -> Response>,
}

impl Router {
    pub fn new(host: &str, port: &str) -> Router {
        let host = host.to_string();
        let port = port.to_string();
        let router: HashMap<String, fn(Request) -> Response> = HashMap::new();
        Router { host, port, router }
    }
    pub fn handle_route(&mut self, path: &str, f: fn(Request) -> Response) {
        self.router.insert(String::from(path), f);
    }
    pub fn listen(&self) {
        let listener = TcpListener::bind(format!("{}:{}", self.host, self.port)).unwrap();
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let buf_reader = BufReader::new(&mut stream);
            let http_request: Vec<_> = buf_reader
                .lines()
                .map(|result| result.unwrap())
                .take_while(|line| !line.is_empty())
                .collect();
            println!("request: {:#?}", http_request);

            let path: Vec<_> = http_request[0].split(" ").collect();
            let handler = self.router.get(path[1]).unwrap();
            let response = handler(Request {});
            let responsebytes = format!(
                "{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
                response.status, response.datatype, response.length, response.contents
            );
            stream.write_all(responsebytes.as_bytes()).unwrap();
        }
    }
}

impl Response {
    pub fn html(contents: String) -> Response {
        let status = String::from("HTTP/1.1 200 Ok");
        let length = contents.len();
        let datatype = String::from("text/html");
        Response {
            status,
            datatype,
            length,
            contents,
        }
    }
    pub fn json<T: std::fmt::Debug>(contents: T) -> Response {
        let typ = std::any::type_name::<T>()
            .replace("rudder::", "")
            .replace("alloc::vec::Vec<", "")
            .replace(">", "");
        println!("{}", typ);
        let oldcontents = format!("{:#?}", contents).replace(&typ, "");
        let mut contents = String::from(" ");
        for c in oldcontents.chars() {
            match c {
                '[' => {
                    contents.push_str("[");
                }
                '{' => {
                    if contents.chars().last().unwrap() == '"' {
                        contents.pop();
                    }
                    contents.push_str("{");
                    contents.push_str("\"");
                }
                '}' => {
                    contents.pop();
                    contents.pop();
                    contents.push_str("}");
                }
                ']' => {
                    contents.pop();
                    contents.pop();
                    contents.push_str("]");
                }
                '"' => {
                    contents.push_str("\"");
                }
                ',' => {
                    contents.push_str(",");
                    contents.push_str("\"");
                }
                ':' => {
                    contents.push_str("\"");
                    contents.push_str(":");
                }
                ' ' => {}
                '\n' => {}
                _ => {
                    contents.push_str(&String::from(c));
                }
            }
        }
        let status = String::from("HTTP/1.1 200 Ok");
        let length = contents.len();
        let datatype = String::from("application/json");
        Response {
            status,
            datatype,
            length,
            contents,
        }
    }
}
