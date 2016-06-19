use std::any::Any;
use hyper::net::Fresh;
use hyper::server::Response as UnwrappedResponse;

pub struct Response<'a, T: Any = Fresh> {
    res: UnwrappedResponse<'a, T>
}

impl<'a> Response<'a, Fresh> {
    pub fn wrap_response<'b>(res: UnwrappedResponse<'b, Fresh>) -> Response<'b, Fresh> {
        Response { res: res }
    }

    pub fn send(self, body: &[u8]) {
        self.res.send(body).unwrap();
    }
}
