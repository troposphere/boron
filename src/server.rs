use hyper::method::Method;
use hyper::server::{Handler, Server, Listening};
use hyper::server::Request as UnwrappedRequest;
use hyper::server::Response as UnwrappedResponse;
use url::Url;
use request::Request;
use response::Response;
use router::{HttpMethods, Router};
use middleware::Middleware;

struct RequestHandler {
    base_url: Url,
    router: Router
}

impl Handler for RequestHandler {
    fn handle<'a, 'k>(&'a self, req: UnwrappedRequest<'a, 'k>, res: UnwrappedResponse<'a>) {
        let boron_req = Request::wrap_request(req, &self.base_url);
        let boron_res = Response::wrap_response(res);
        self.router.serve(boron_req, boron_res);
    }
}

pub struct Boron {
    server: Option<Listening>,
    router: Router
}

impl Boron {
    pub fn new() -> Boron {
        Boron {
            server: None,
            router: Router::new("".to_string())
        }
    }

    pub fn listen(mut self, host_port: &str) {
        let handler = RequestHandler {
            base_url: Url::parse(format!("http://{}", host_port).as_str()).unwrap(),
            router: self.router
        };
        let server = Server::http(host_port)
            .unwrap()
            .handle(handler)
            .unwrap();
        self.server = Some(server);
    }
}

impl HttpMethods for Boron {
    fn new_route<T: Middleware>(&mut self, method: Method, path: &str, action: T) {
        self.router.new_route(method, path, action);
    }
}
