use rudder::*;
use std::fs;

fn index() -> String {
    fs::read_to_string("templates/index.html").unwrap()
}

fn test() -> String {
    fs::read_to_string("templates/test.html").unwrap()
}

fn main() {
    let mut app = rudder::App::new("127.0.0.1","7878");

    app.add_path("/",index);
    app.add_path("/test",test);

    app.run();
}