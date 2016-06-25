extern crate hyper;
extern crate tungsten;

use std::thread;
use std::io::Read;
use hyper::status::StatusCode;
use hyper::client::Client;
use tungsten::server::Tungsten;
use tungsten::request::Request;
use tungsten::response::Response;
use tungsten::router::HttpMethods;

fn serve() {
  let _ = thread::Builder::new().name(String::from("test-server")).spawn(move || {
    let mut app = Tungsten::new();
    app.get("/".to_string(), |req: Request, res: Response| {
        res.send(b"Hello World!");
    });
    app.get("/some/random/path".to_string(), |req: Request, res: Response| {
        res.send(b"You are at /some/random/path");
    });
    app.listen("0.0.0.0:4040");
  });
}

#[test]
fn test_hello_world() {
    serve();
    let client = Client::new();
    let mut res = client.get("http://0.0.0.0:4040").send().unwrap();
    let mut body = String::new();
    let _ = res.read_to_string(&mut body);

    assert_eq!(res.status, StatusCode::Ok);
    assert_eq!(body, "Hello World!");
}

#[test]
fn test_some_path() {
    let client = Client::new();
    let mut res = client.get("http://0.0.0.0:4040/some/random/path").send().unwrap();
    let mut body = String::new();
    let _ = res.read_to_string(&mut body);

    assert_eq!(res.status, StatusCode::Ok);
    assert_eq!(body, "You are at /some/random/path");
}
