//! Boron app server methods and utilities

use hyper::method::Method;
use hyper::server::{Handler as HyperHandler, Server, Listening};
use hyper::server::Request as UnwrappedRequest;
use hyper::server::Response as UnwrappedResponse;
use url::Url;
use request::Request;
use response::Response;
use router::{HttpMethods, Router};
use middleware::*;

struct RequestHandler {
    base_url: Url,
    router: Router
}

impl HyperHandler for RequestHandler {
    fn handle<'a, 'k>(&'a self, req: UnwrappedRequest<'a, 'k>, res: UnwrappedResponse<'a>) {
        let boron_req = Request::wrap_request(req, &self.base_url);
        let boron_res = Response::wrap_response(res);
        self.router.serve(boron_req, boron_res);
    }
}

/// This acts as the central application object and is the starting point for a Boron application.
/// Once instantiated it can be used to add different URL matching routes and middleware logic for
/// your app.
pub struct Boron {
    server: Option<Listening>,
    router: Router
}

impl Boron {

    /// Creates a new Boron object.
    ///
    /// Generally it is created in the `main` function of your program liken this:
    ///
    /// ```
    /// use boron::server::Boron;
    ///
    /// fn main() {
    ///     let mut app = Boron::new();
    /// }
    /// ```
    pub fn new() -> Boron {
        Boron {
            server: None,
            router: Router::new("".to_string())
        }
    }

    /// Starts the application server on a specified host port. The host and port are passed as a
    /// string in a manner similar to `localhost:4000`.
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
    fn new_route<T: Handler>(&mut self, method: Method, path: &str, action: T) {
        self.router.new_route(method, path, action);
    }

    fn use_before<T: BeforeMiddleware>(&mut self, path: &str, action: T) {
        self.router.use_before(path, action);
    }

    fn use_after<T: AfterMiddleware>(&mut self, path: &str, action: T) {
        self.router.use_after(path, action);
    }
}
