## rudder

Rudder is a web framework built from scratch in Rust. At the moment project is in it's early design and prototyping stage. To run it on your own server you first need to clone the repository:

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