//! Traits to implement middleware types.
//!
//! This module defined the middleware traits for the closures that would be used as handlers and
//! middlewares in the application. This provides nice compile time guarantees of the what each
//! specific middleware can do and return. If anything else is returned then it results in a
//! compile time error.

use std::io;
use hyper::net::Fresh;
use request::Request;
use response::{Response, ShadowResponse};

/// Implements the kind of middleware that is executed when a request has arrived but before it is
/// passed on to the handler for being processed.
///
/// You have access to the request here and can do some pre-processing as required, but you cannot
/// write a response from a `BeforeMiddleware`. You need to implement a `Handler` to do that.
///
/// Typical uses for this middleware is for checking certain header or some form of authentication
/// on the request before it is served by the application.
pub trait BeforeMiddleware: Send + Sync + 'static {
    fn execute<'m, 'r>(&'m self, req: &Request<'m, 'r>);
}

impl<T> BeforeMiddleware for T where T: for <'m, 'r> Fn(&Request<'m, 'r>) + Send + Sync + 'static {
    fn execute<'m, 'r>(&'m self, req: &Request<'m, 'r>) {
        (*self)(req);
    }
}

/// The actual logic for handling the request and writing the response is a part of this
/// middleware.
///
/// Here you have access to both the request and response objects and you need to write the
/// response for a particular request in the Handler type itself. Execution of a `Handler` returns
/// a `ShadowResponse` which is passed on to the `AfterMiddleware`.
pub trait Handler: Send + Sync + 'static {
    fn execute<'m, 'r>(&'m self, req: &Request<'m, 'r>, res: Response<'m, Fresh>) -> io::Result<ShadowResponse>;
}

impl<T> Handler for T where T: for <'m, 'r> Fn(&Request<'m, 'r>, Response<'m>) -> io::Result<ShadowResponse> + Send + Sync + 'static {
    fn execute<'m, 'r>(&'m self, req: &Request<'m, 'r>, res: Response<'m>) -> io::Result<ShadowResponse> {
        (*self)(req, res)
    }
}

/// Any logic to be executed per request after the response has been sent is a part of this
/// middleware.
///
/// This has access to the `Request` and `ShadowResponse` objects.
///
/// Typical uses for this middleware could be in logging something after the request is served.
pub trait AfterMiddleware: Send + Sync + 'static {
    fn execute<'m, 'r>(&'m self, req: &Request<'m, 'r>, res: &ShadowResponse);
}

impl<T> AfterMiddleware for T where T: for <'m, 'r> Fn(&Request<'m, 'r>, &ShadowResponse) + Send + Sync + 'static {
    fn execute<'m, 'r>(&'m self, req: &Request<'m, 'r>, res: &ShadowResponse) {
        (*self)(req, res);
    }
}
