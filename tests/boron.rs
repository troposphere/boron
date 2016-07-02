extern crate hyper;
extern crate boron;

use std::thread;
use std::sync::{Once, ONCE_INIT};
use std::io::{Read, Write};
use hyper::status::StatusCode;
use hyper::client::Client;
use hyper::client::response::Response as HyperResponse;
use boron::server::Boron;
use boron::request::Request;
use boron::response::Response;
use boron::router::HttpMethods;

static TEST_INIT: Once = ONCE_INIT;

struct TestContext {
    req_client: Client
}

impl TestContext {
    fn new() ->  TestContext {
        let ctx = TestContext {
            req_client: Client::new()
        };
        TEST_INIT.call_once(|| {
            let _ = thread::spawn(move || {
                let mut app = Boron::new();
                app.get("/".to_string(), |req: Request, res: Response| {
                    res.send(b"Hello World!");
                });
                app.get("/some/random/path".to_string(), |req: Request, res: Response| {
                    res.send(b"You are at /some/random/path");
                });
                app.get("/throw/error".to_string(), |req: Request, mut res: Response| {
                    *res.status_mut() = StatusCode::InternalServerError;
                    let mut started_res = res.start().unwrap();
                    started_res.write(b"Boom!");
                    started_res.end();
                });
                app.listen("0.0.0.0:4040");
            });
            loop {
                if ctx.req_client.get("http://0.0.0.0:4040").send().is_ok() {
                    break;
                }
            }
        });
        ctx
    }

    fn request(&self, url: &str) -> HyperResponse {
        self.req_client.get(url).send().unwrap()
    }

    fn body_from_response(&self, res: &mut HyperResponse) -> String {
        let mut body = String::new();
        let _ = res.read_to_string(&mut body);
        body
    }
}

#[test]
fn test_hello_world() {
    let ctx = TestContext::new();
    let mut res = ctx.request("http://0.0.0.0:4040");
    let body = ctx.body_from_response(&mut res);

    assert_eq!(res.status, StatusCode::Ok);
    assert_eq!(body, "Hello World!");
}

#[test]
fn test_some_path() {
    let ctx = TestContext::new();
    let mut res = ctx.request("http://0.0.0.0:4040/some/random/path");
    let body = ctx.body_from_response(&mut res);

    assert_eq!(res.status, StatusCode::Ok);
    assert_eq!(body, "You are at /some/random/path");
}

#[test]
fn test_res_methods() {
    let ctx = TestContext::new();
    let mut res = ctx.request("http://0.0.0.0:4040/throw/error");
    let body = ctx.body_from_response(&mut res);

    assert_eq!(res.status, StatusCode::InternalServerError);
    assert_eq!(body, "Boom!");
}
