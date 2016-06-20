use middleware::Middleware;

struct Route {
    path: String,
    action: Box<Middleware>
}

trait HttpMethods {
    fn get<T: Middleware>(&mut self, String, T);
}

struct Router {
    base: String,
    routes: Vec<Route>
}

impl Router {
    fn new(base: String) -> Router {
        Router {
            base: base,
            routes: vec![]
        }
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
