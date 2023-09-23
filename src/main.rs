use rudder::*;
use std::fs;

const template : &str = "<h1>test template</h1>";

fn controller() -> rudder::Response {
    let response = rudder::Response::HTMLstring(String::from(template));
    response
}

fn main() {
    let mut app = rudder::App::new("127.0.0.1","7878");
    app.handle_route("/",controller);
    app.listen();
}