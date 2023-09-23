use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use std::collections::HashMap;

pub struct App {
    host:String,
    port:String,
    router:HashMap<String,fn()->Response>,
}

pub struct Response {
    status:String,
    length:usize,
    contents:String,
}

impl App {
    pub fn new(host:&str,port:&str) -> App {
        let host = host.to_string();
        let port = port.to_string();
        let router: HashMap<String,fn()->Response> = HashMap::new();
        App{host,port,router}
    }
    pub fn handle_route(&mut self,path:&str,f:fn()->Response) {
        self.router.insert(String::from(path),f);
    }
    pub fn listen(&self) {
        let listener = TcpListener::bind(format!("{}:{}",self.host,self.port)).unwrap();
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let buf_reader = BufReader::new(&mut stream);
            let http_request: Vec<_> = buf_reader.lines().map(|result| result.unwrap()).take_while(|line| !line.is_empty()).collect();
            println!("request: {:#?}",http_request);

            let path: Vec<_> = http_request[0].split(" ").collect();
            let handler = self.router.get(path[1]).unwrap();
            let response = handler();
            let responsebytes = format!("{}\r\nContent-Length: {}\r\n\r\n{}",response.status,response.length,response.contents);
            stream.write_all(responsebytes.as_bytes()).unwrap();
        }
    }
}

impl Response {
    pub fn HTMLstring(contents:String) -> Response {
        let status = String::from("HTTP/1.1 200 Ok");
        let length = contents.len();
        Response{status,length,contents}
    }
}