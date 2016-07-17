use std::io::{self, Write};
use std::any::Any;
use hyper::net::{Fresh, Streaming};
use hyper::server::Response as UnwrappedResponse;
use hyper::header::Headers;
use hyper::status::StatusCode;

pub struct Response<'a, T: Any = Fresh> {
    res: UnwrappedResponse<'a, T>
}

pub struct ShadowResponse {
    pub status: StatusCode,
    pub headers: Headers
}

impl<'a> Response<'a, Fresh> {
    pub fn wrap_response<'b>(res: UnwrappedResponse<'b, Fresh>) -> Response<'b, Fresh> {
        Response { res: res }
    }

    #[inline]
    pub fn status_mut(&mut self) -> &mut StatusCode {
        self.res.status_mut()
    }

    #[inline]
    pub fn headers_mut(&mut self) -> &mut Headers {
        self.res.headers_mut()
    }

    #[inline]
    pub fn send(self, body: &[u8]) -> io::Result<ShadowResponse> {
        let shadow_res = ShadowResponse {
            status: self.res.status().clone(),
            headers: self.res.headers().clone()
        };

        match self.res.send(body) {
            Ok(()) => Ok(shadow_res),
            Err(e) => Err(e)
        }
    }

    #[inline]
    pub fn start(self) -> io::Result<Response<'a, Streaming>> {
        match self.res.start() {
            Ok(res) => Ok(Response { res: res }),
            Err(e) => Err(e)
        }
    }
}

impl<'a> Response<'a, Streaming> {
    #[inline]
    pub fn end(self) -> io::Result<ShadowResponse> {
        let shadow_res = ShadowResponse {
            status: self.res.status().clone(),
            headers: self.res.headers().clone()
        };

        match self.res.end() {
            Ok(()) => Ok(shadow_res),
            Err(e) => Err(e)
        }
    }
}

impl<'a> Write for Response<'a, Streaming> {
    #[inline]
    fn write(&mut self, msg: &[u8]) -> io::Result<usize> {
        self.res.write(msg)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.res.flush()
    }
}
