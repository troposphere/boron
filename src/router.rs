use middleware::Middleware;
use request::Request;
use response::Response;

struct Route {
    path: String,
    action: Box<Middleware>
}

pub trait HttpMethods {
    fn get<T: Middleware>(&mut self, String, T);
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
        assert!(self.routes.len() > 0);
        match self.match_route(req.path().unwrap().to_string()) {
            Some(route) => route.action.execute(req, res),
            None => panic!("Route not found.")
        };
    }

    fn match_route(&self, path: String) -> Option<&Route> {
        let mut matched_route = None;
        for route in self.routes.iter() {
            if route.path == path.to_string() {
                matched_route = Some(route);
                break;
            }
        }
        matched_route
    }
}

impl HttpMethods for Router {
    fn get<T: Middleware>(&mut self, path: String, action: T) {
        let route = Route {
            path: path,
            action: Box::new(action)
        };
        self.routes.push(route);
    }
}
