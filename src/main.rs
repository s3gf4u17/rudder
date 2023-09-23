use rudder::*;
use std::fs;

const template: &str = "<h1>test template for user: {{name}}</h1>";

struct User {
    name: String,
}

fn controller() -> rudder::Response {
    let response = rudder::Response::HTMLstring(String::from(template));
    let user = User {
        name: String::from("Alisson"),
    };
    let response = rudder::Response::HTMLdynamic::<User>(String::from(template), user);
    response
}

fn main() {
    let mut app = rudder::App::new("127.0.0.1", "7878");
    app.handle_route("/", controller);
    app.listen();
}
