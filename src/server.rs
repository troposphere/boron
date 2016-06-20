use hyper::server::{Handler, Server, Listening};
use request::Request;
use hyper::server::Request as UnwrappedRequest;
use hyper::server::Response as UnwrappedResponse;
use response::Response;

struct RequestHandler;

impl Handler for RequestHandler {
    fn handle(&self, req: UnwrappedRequest, res: UnwrappedResponse) {
        let _tungsten_req = Request::wrap_request(req);
        let tungsten_res = Response::wrap_response(res);
        tungsten_res.send(b"Hello World!");
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
