use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use std::collections::HashMap;

pub struct App {
    host:String,
    port:String,
    paths:HashMap<String,fn()->String>,
}

impl App {
    pub fn new(host:&str,port:&str) -> App {
        let host = host.to_string();
        let port = port.to_string();
        let paths:HashMap<String,fn()->String> = HashMap::new();
        App{host,port,paths}
    }
    pub fn add_path(&mut self,path:&str,f:fn()->String) {
        self.paths.insert(String::from(path),f);
    }
    pub fn run(&self) {
        let listener = TcpListener::bind(format!("{}:{}",self.host,self.port)).unwrap();
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();

            let buf_reader = BufReader::new(&mut stream);
            let http_request: Vec<_> = buf_reader.lines().map(|result| result.unwrap()).take_while(|line| !line.is_empty()).collect();
            // println!("request: {:#?}",http_request);
            let path: Vec<_> = http_request[0].split(" ").collect();
            println!("request: {:#?}",path[1]);

            let func = self.paths.get(path[1]).unwrap();
            let contents = func();
            let status_line = "HTTP/1.1 200 OK";
            let length = contents.len();
            let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
            stream.write_all(response.as_bytes()).unwrap();
        }
    }
}