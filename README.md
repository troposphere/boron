# Boron [![Build Status](https://travis-ci.org/troposphere/boron.svg?branch=master)](https://travis-ci.org/troposphere/boron)

A web framework made with ❤

Boron was born as an attempt to learn Rust as we go along making a web framework. It aims to be a fast and minimalist web framework.

This is a work in progress and expect a lot of things to break at the moment.

## Create your first app

```rust
extern crate boron;

use boron::server::Boron;
use boron::request::Request;
use boron::response::Response;
use boron::router::HttpMethods;

fn main() {
    let mut app = Boron::new();
    app.get("/", |req: Request, res: Response| {
        res.send(b"Hello World! I am Boron.");
    });
    app.listen("localhost:3000");
}
```

Add the following line to your `[dependencies]` section in `Cargo.toml`:

```
boron = "0.0.2"
```

## Contributing

You want to contribute? Awesome! We need help.

Fork the repo, start hacking away and send us your pull requests. We maintain a list of things to do on the [issues page](https://github.com/troposphere/boron/issues).

## License

MIT

