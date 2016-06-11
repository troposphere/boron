extern crate hyper;
extern crate tungsten;

use std::thread;
use std::io::Read;
use hyper::status::StatusCode;
use hyper::client::Client;
use tungsten::tungsten::Tungsten;

fn serve() {
  thread::Builder::new().name(String::from("test-server")).spawn(move || {
    let mut server = Tungsten::new();
    server.listen("0.0.0.0:4040");
  });
}

#[test]
fn test_hello_world() {
    serve();
    let client = Client::new();
    let mut res = client.get("http://0.0.0.0:4040").send().unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body);

    assert_eq!(res.status, StatusCode::Ok);
    assert_eq!(body, "Hello World!");
}
