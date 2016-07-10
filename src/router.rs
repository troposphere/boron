use middleware::Middleware;
use request::Request;
use response::Response;
use matcher::Matcher;
use hyper::method::Method;

struct Route {
    method: Method,
    path: Matcher,
    action: Box<Middleware>
}

pub trait HttpMethods {
    fn new_route<T: Middleware>(&mut self, Method, &str, T);

    fn get<T: Middleware>(&mut self, path: &str, action: T) {
        self.new_route(Method::Get, path, action);
    }

    fn post<T: Middleware>(&mut self, path: &str, action: T) {
        self.new_route(Method::Post, path, action);
    }

    fn put<T: Middleware>(&mut self, path: &str, action: T) {
        self.new_route(Method::Put, path, action);
    }

    fn delete<T: Middleware>(&mut self, path: &str, action: T) {
        self.new_route(Method::Delete, path, action);
    }

    fn utilize<T: Middleware>(&mut self, path: &str, action: T) {
        self.new_route(Method::Extension("BoronMiddleware".to_string()), path, action);
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
        match self.match_route(req.method(), req.path()) {
            Some(routes) => {
                for handler in routes {
                    handler.action.execute(req, res);
                }
            },
            None => panic!("Route not found.")
        };
    }

    fn match_route(&self, method: &Method, path: &str) -> Option<Vec<&Route>> {
        let mut matched_routes: Vec<&Route> = vec![];
        let mut url_handler: Option<&Route> = None;

        for route in self.routes.iter() {
            if route.path.is_match(path) {
                if route.method == *method {
                    url_handler = Some(route);
                } else if route.method == Method::Extension("BoronMiddleware".to_string()) {
                    matched_routes.push(route);
                }
            }
        }
        match url_handler {
            Some(handler) => {
                matched_routes.push(handler);
                Some(matched_routes)
            },
            None => None,
        }
    }
}

impl HttpMethods for Router {
    fn new_route<T: Middleware>(&mut self, method: Method, path: &str, action: T) {
        let route = Route {
            method: method,
            path: Matcher::new(path),
            action: Box::new(action)
        };
        self.routes.push(route);
    }
}
