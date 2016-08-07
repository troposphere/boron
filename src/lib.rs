//! Boron is a small and expressive web framework for Rust which aims to give a robust foundation
//! for web applications and APIs.
//!
//! ## Installation
//! Add the following line to your `[dependecies]` section in `Cargo.toml`:
//!
//! ```toml
//! boron = "0.0.2"
//! ```
//!
//! ## Your first app
//!
//! ```rust,no_run
//! extern crate boron;
//!
//! use boron::server::Boron;
//! use boron::request::Request;
//! use boron::response::Response;
//! use boron::router::HttpMethods;
//!
//! fn main() {
//!     let mut app = Boron::new();
//!     app.get("/", |req: &Request, res: Response| {
//!         res.send(b"Hello World! I am Boron.")
//!     });
//!    app.listen("localhost:3000");
//! }
//! ```

extern crate hyper;
extern crate url;
extern crate regex;
extern crate typemap;

pub mod server;
pub mod response;
pub mod request;
pub mod middleware;
pub mod router;
mod matcher;
