# Rudder

Rudder is a high-performance web framework for building web applications in Rust. It provides a set of tools and features to help you create web applications with ease. Rudder supports creating RESTful APIs as well as full stack web applications. At the moment project is in it's early design and prototyping stage.

### Getting Started

To start using Rudder, first you need to clone this repository:

```
git clone https://github.com/s3gf4u17/rudder
```

Then you can develop your service by configuring `main.rs` file to your needs. At the moment there is a working example set up, showing how to use Rudder features:

```rust
use rudder::{Request, Response, Router}; // import Rudder functionality

// create a data struct for generating json response
#[derive(Debug)]
struct User {
    name: String,
    age: u8,
}

// response handler for '127.0.0.1:7878/html'
fn htmlSite(request: Request) -> Response {
    Response::html(String::from("hello rudder!")) // generate response from string
}

// response handler for '127.0.0.1:7878/json'
fn jsonSite(request: Request) -> Response {
    Response::json(User {
        name: String::from("alisson"),
        age: 17,
    }) // generate response from user object
}

fn main() {
    let mut router = Router::new("127.0.0.1", "7878"); // initialize router (host,port)
    router.handle_route("/html", htmlSite); // define route handler function
    router.handle_route("/json", jsonSite); // define route handler function
    router.listen(); // run router
}
```

### Notes

At the moment using structs as arguments to generate `Response::json` requires them to have default Rust Debug trait derived.