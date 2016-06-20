use hyper::server::Request as UnwrappedRequest;
use hyper::uri::RequestUri::AbsolutePath;

pub struct Request<'a, 'b: 'a> {
    pub req: UnwrappedRequest<'a, 'b>
}

impl<'a, 'b> Request<'a, 'b> {
    pub fn wrap_request(req: UnwrappedRequest<'a, 'b>) -> Request<'a, 'b> {
        Request {
            req: req
        }
    }

    pub fn path(&self) -> Option<&str> {
        match self.req.uri {
            AbsolutePath(ref path) => Some(path.splitn(2, '?').next().unwrap()),
            _ => None
        }
    }
}
