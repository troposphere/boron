use hyper::server::{Handler, Server, Listening};
use hyper::server::Request as UnwrappedRequest;
use hyper::server::Response as UnwrappedResponse;

struct RequestHandler;

impl Handler for RequestHandler {
    fn handle(&self, _req: UnwrappedRequest, res: UnwrappedResponse) {
        res.send(b"Hello World!").unwrap();
    }
}

pub struct Tungsten {
    server: Option<Listening>
}

impl Tungsten {
    pub fn new() -> Tungsten {
        Tungsten { server: None }
    }

    pub fn listen(&mut self, host_port: &str) {
        let server = Server::http(host_port)
            .unwrap()
            .handle(RequestHandler)
            .unwrap();
        self.server = Some(server);
    }
}
