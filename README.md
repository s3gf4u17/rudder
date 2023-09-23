## rudder

Rudder is a web framework built from scratch in Rust. Project inspired by the design of Django. At the moment project is in it's early design and prototyping stage. To run it on your own server you first need to clone the repository:

```
git clone https://github.com/s3gf4u17/rudder
```

And then run or edit code in `main.rs`:

```rust
use rudder::*;

fn main() {
    let app = rudder::App::new("127.0.0.1","7878");
    app.run();
}
```

### Examples

Simple routing:

```rust
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
```