use hyper::net::Fresh;
use request::Request;
use response::{Response, ShadowResponse};

pub trait BeforeMiddleware: Send + Sync + 'static {
    fn execute<'m, 'r>(&'m self, req: &Request<'m, 'r>);
}

impl<T> BeforeMiddleware for T where T: for <'m, 'r> Fn(&Request<'m, 'r>) + Send + Sync + 'static {
    fn execute<'m, 'r>(&'m self, req: &Request<'m, 'r>) {
        (*self)(req);
    }
}

pub trait Handler: Send + Sync + 'static {
    fn execute<'m, 'r>(&'m self, req: &Request<'m, 'r>, res: Response<'m, Fresh>);
}

impl<T> Handler for T where T: for <'m, 'r> Fn(&Request<'m, 'r>, Response<'m>) + Send + Sync + 'static {
    fn execute<'m, 'r>(&'m self, req: &Request<'m, 'r>, res: Response<'m>) {
        (*self)(req, res);
    }
}

pub trait AfterMiddleware: Send + Sync + 'static {
    fn execute<'m, 'r>(&'m self, req: &Request<'m, 'r>/*, res: ShadowResponse*/);
}

impl<T> AfterMiddleware for T where T: for <'m, 'r> Fn(&Request<'m, 'r>/*, ShadowResponse*/) + Send + Sync + 'static {
    fn execute<'m, 'r>(&'m self, req: &Request<'m, 'r>/*, res: ShadowResponse*/) {
        (*self)(req/*, res*/);
    }
}
