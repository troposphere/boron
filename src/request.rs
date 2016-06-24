use hyper::server::Request as UnwrappedRequest;
use hyper::uri::RequestUri::AbsolutePath;
use hyper::method::Method;
use url::{Url, ParseError};

pub struct Request<'a, 'b: 'a> {
    pub req: UnwrappedRequest<'a, 'b>,
    parsed_url: Url
}

impl<'a, 'b> Request<'a, 'b> {
    pub fn wrap_request(req: UnwrappedRequest<'a, 'b>, base_url: &Url) -> Request<'a, 'b> {
        let request_url = match req.uri {
            AbsolutePath(ref path) => base_url.join(path.as_str()),
            _ => Err(ParseError::RelativeUrlWithoutBase)
        };
        Request {
            req: req,
            parsed_url: request_url.unwrap()
        }
    }

    pub fn path(&self) -> &str {
        self.parsed_url.path()
    }

    pub fn method(&self) -> &Method {
        &self.req.method
    }
}
