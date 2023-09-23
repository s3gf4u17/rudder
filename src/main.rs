use rudder::{Request, Response, Router};

#[derive(Debug)]
struct User {
    name: String,
    age: u8,
}

fn htmlsite(request: Request) -> Response {
    Response::html(String::from("hello rudder!"))
}

fn jsonsite(request: Request) -> Response {
    Response::json(User {
        name: String::from("alisson"),
        age: 17,
    })
}

fn main() {
    let mut router = Router::new("127.0.0.1", "7878");
    router.handle_route("/html", htmlsite);
    router.handle_route("/json", jsonsite);
    router.listen();
}
