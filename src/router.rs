use middleware::Middleware;
use request::Request;
use response::Response;
use hyper::method::Method;

struct Route {
    method: Method,
    path: String,
    action: Box<Middleware>
}

pub trait HttpMethods {
    fn new_route<T: Middleware>(&mut self, Method, String, T);
    fn get<T: Middleware>(&mut self, path: String, action: T) {
        self.new_route(Method::Get, path, action);
    }

    fn post<T: Middleware>(&mut self, path: String, action: T) {
        self.new_route(Method::Post, path, action);
    }

    fn put<T: Middleware>(&mut self, path: String, action: T) {
        self.new_route(Method::Put, path, action);
    }

    fn delete<T: Middleware>(&mut self, path: String, action: T) {
        self.new_route(Method::Delete, path, action);
    }
}

pub struct Router {
    base: String,
    routes: Vec<Route>
}

impl Router {
    pub fn new(base: String) -> Router {
        Router {
            base: base,
            routes: vec![]
        }
    }

    pub fn serve<'m, 'r>(&'m self, req: Request<'m, 'r>, res: Response<'m>) {
        match self.match_route(req.method(), req.path().to_string()) {
            Some(route) => route.action.execute(req, res),
            None => panic!("Route not found.")
        };
    }

    fn match_route(&self, method: &Method, path: String) -> Option<&Route> {
        let mut matched_route = None;
        for route in self.routes.iter() {
            if route.method == *method && route.path == path.to_string() {
                matched_route = Some(route);
                break;
            }
        }
        matched_route
    }
}

impl HttpMethods for Router {
    fn new_route<T: Middleware>(&mut self, method: Method, path: String, action: T) {
        let route = Route {
            method: method,
            path: path,
            action: Box::new(action)
        };
        self.routes.push(route);
    }
}
