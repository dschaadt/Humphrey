<p align="center">
  <img src="https://raw.githubusercontent.com/w-henderson/Humphrey/master/assets/logo.png" width=250><br><br>
  <img src="https://img.shields.io/badge/language-rust-b07858?style=for-the-badge&logo=rust" style="margin-right:5px">
  <img src="https://img.shields.io/github/workflow/status/w-henderson/Humphrey/CI?style=for-the-badge" style="margin-right:5px">
  <img src="https://img.shields.io/crates/v/humphrey?style=for-the-badge" style="margin-right:5px">
</p>

# Humphrey: A Performance-Focused, Dependency-Free Web Server.
Humphrey is a very fast, robust and flexible HTTP/1.1 web server crate which allows you to develop web applications in Rust. With no dependencies, it is very quick to compile and produces very small binaries, as well as being very resource-efficient.

## Installation
The Humphrey crate can be installed by adding `humphrey` to your Cargo.toml file.

## Documentation
The Humphrey documentation can be found at [docs.rs](https://docs.rs/humphrey).

## Basic Example
```rs
use humphrey::http::{Request, Response, StatusCode};
use humphrey::App;
use std::{error::Error, sync::Arc};

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new()
        .with_route("/", home)
        .with_route("/contact", contact);
    app.run("0.0.0.0:80")?;

    Ok(())
}

fn home(request: Request, _: Arc<()>) -> Response {
    Response::new(StatusCode::OK, b"<html><body><h1>Home</h1></body></html>", &request)
}

fn contact(request: Request, _: Arc<()>) -> Response {
    Response::new(StatusCode::OK, b"<html><body><h1>Contact</h1></body></html>", &request)
}
```

## Further Examples
- [Stateful Example](https://github.com/w-henderson/Humphrey/tree/master/examples/stateful): keeps track of button presses across sessions and devices
- [Wildcard Example](https://github.com/w-henderson/Humphrey/tree/master/examples/wildcard): demonstrates a wildcard route
- [Static Content Example](https://github.com/w-henderson/Humphrey/tree/master/examples/static-content): demonstrates the built-in static content handlers