use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

pub struct App {
    host:String,
    port:String,
}

impl App {
    pub fn new(host:&str,port:&str) -> App {
        let host = host.to_string();
        let port = port.to_string();
        App{host,port}
    }
    pub fn run(&self) {
        let listener = TcpListener::bind(format!("{}:{}",self.host,self.port)).unwrap();
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            handle_connection(stream);
        }
    }
}

pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
    .lines().map(|result| result.unwrap()).take_while(|line| !line.is_empty()).collect();
    println!("request: {:#?}",http_request);

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("index.html").unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}