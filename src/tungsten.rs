use hyper::status::StatusCode;
use hyper::client::Client;
use hyper::server::{Server, Request, Response};

pub struct Tungsten {
    server: Option<Server>
}

impl Tungsten {
    pub fn new() -> Tungsten {
        Tungsten { server: None }
    }

    pub fn listen(&self, host_port: &str) {
        Server::http(host_port)
            .unwrap()
            .handle(|_req: Request, res: Response| {
                res.send(b"Hello World!").unwrap();
            })
            .unwrap();
    }
}

#[test]

fn test_hello_world() {
    let server = Tungsten::new();
    server.listen("0.0.0.0:4040");

    // The following lines don't actually work. :P
    // Right now I checked with curl
    let client = Client::new();
    println!("Server is up");
    let res = client.get("http://0.0.0.0:4040").send().unwrap();
    assert_eq!(res.status, StatusCode::Ok);
}
