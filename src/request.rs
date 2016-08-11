//! HTTP request representation in Boron

use std::io::{self, Read};
use std::net::SocketAddr;
use std::time::Duration;
use hyper::server::Request as UnwrappedRequest;
use hyper::uri::RequestUri::AbsolutePath;
use hyper::method::Method;
use hyper::header::Headers;
use url::{Url, ParseError};
use typemap::TypeMap;

pub struct Request<'a, 'b: 'a> {
    pub req: UnwrappedRequest<'a, 'b>,
    parsed_url: Url,
    pub url_tokens: Vec<(String, String)>,
    pub extensions: TypeMap
}

impl<'a, 'b> Request<'a, 'b> {
    pub fn wrap_request(req: UnwrappedRequest<'a, 'b>, base_url: &Url) -> Request<'a, 'b> {
        let request_url = match req.uri {
            AbsolutePath(ref path) => base_url.join(path.as_str()),
            _ => Err(ParseError::RelativeUrlWithoutBase)
        };
        Request {
            req: req,
            parsed_url: request_url.unwrap(),
            url_tokens: vec![],
            extensions: TypeMap::new()
        }
    }

    pub fn url_param(&self, key: &str) -> Option<&str> {
        for token in &self.url_tokens {
            if token.0 == key {
                return Some(token.1.as_str());
            }
        }
        None
    }

    #[inline]
    pub fn path(&self) -> &str {
        self.parsed_url.path()
    }

    #[inline]
    pub fn method(&self) -> &Method {
        &self.req.method
    }

    #[inline]
    pub fn remote_addr(&self) -> &SocketAddr {
        &self.req.remote_addr
    }

    #[inline]
    pub fn headers(&self) -> &Headers {
        &self.req.headers
    }

    #[inline]
    pub fn set_read_timeout(&self, timeout: Option<Duration>) -> io::Result<()> {
        self.req.set_read_timeout(timeout)
    }
}

impl<'a, 'b> Read for Request<'a, 'b> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.req.read(buf)
    }
}
