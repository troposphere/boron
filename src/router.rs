//! Router logic to dispatch requests

use middleware::*;
use request::Request;
use response::Response;
use matcher::Matcher;
use hyper::method::Method;

struct Route {
    method: Method,
    path: Matcher,
    action: Box<Handler>
}

struct BeforeRoute {
    path: Matcher,
    action: Box<BeforeMiddleware>
}

struct AfterRoute {
    path: Matcher,
    action: Box<AfterMiddleware>
}

pub trait HttpMethods {
    fn new_route<T: Handler>(&mut self, Method, &str, T);

    fn get<T: Handler>(&mut self, path: &str, action: T) {
        self.new_route(Method::Get, path, action);
    }

    fn post<T: Handler>(&mut self, path: &str, action: T) {
        self.new_route(Method::Post, path, action);
    }

    fn put<T: Handler>(&mut self, path: &str, action: T) {
        self.new_route(Method::Put, path, action);
    }

    fn delete<T: Handler>(&mut self, path: &str, action: T) {
        self.new_route(Method::Delete, path, action);
    }

    fn use_before<T: BeforeMiddleware>(&mut self, path: &str, action: T);

    fn use_after<T: AfterMiddleware>(&mut self, path: &str, action: T);
}

pub struct Router {
    base: String,
    handlers: Vec<Route>,
    before_middlewares: Vec<BeforeRoute>,
    after_middlewares: Vec<AfterRoute>
}

impl Router {
    pub fn new(base: String) -> Router {
        Router {
            base: base,
            handlers: vec![],
            before_middlewares: vec![],
            after_middlewares: vec![]
        }
    }

    pub fn serve<'m, 'r>(&'m self, req: Request<'m, 'r>, res: Response<'m>) {
        match self.match_route(req.method(), req.path()) {
            Some(routes) => {
                let (before_middlewares, handler, after_middlewares) = routes;
                for middleware in before_middlewares {
                    middleware.action.execute(&req);
                }
                let shadow_res = handler.action.execute(&req, res).unwrap();
                for middleware in after_middlewares {
                    middleware.action.execute(&req, &shadow_res);
                }
            },
            None => panic!("Route not found.")
        };
    }

    fn match_route(&self, method: &Method, path: &str) -> Option<(Vec<&BeforeRoute>, &Route, Vec<&AfterRoute>)> {
        let mut before_middlewares: Vec<&BeforeRoute> = vec![];
        let mut url_handler: Option<&Route> = None;
        let mut after_middlewares: Vec<&AfterRoute> = vec![];

        for route in self.handlers.iter() {
            if route.path.is_match(path) && route.method == *method {
                url_handler = Some(route);
            }
        }

        if url_handler.is_some() {
            for route in self.before_middlewares.iter() {
                if route.path.is_match(path) {
                    before_middlewares.push(route);
                }
            }

            for route in self.after_middlewares.iter() {
                if route.path.is_match(path) {
                    after_middlewares.push(route);
                }
            }

            Some((before_middlewares, url_handler.unwrap(), after_middlewares))
        } else {
            None
        }
    }
}

impl HttpMethods for Router {
    fn new_route<T: Handler>(&mut self, method: Method, path: &str, action: T) {
        let route = Route {
            method: method,
            path: Matcher::new(path),
            action: Box::new(action)
        };
        self.handlers.push(route);
    }

    fn use_before<T: BeforeMiddleware>(&mut self, path: &str, action: T) {
        let before_route = BeforeRoute {
            path: Matcher::new(path),
            action: Box::new(action)
        };
        self.before_middlewares.push(before_route);
    }

    fn use_after<T: AfterMiddleware>(&mut self, path: &str, action: T) {
        let after_route = AfterRoute {
            path: Matcher::new(path),
            action: Box::new(action)
        };
        self.after_middlewares.push(after_route);
    }
}
