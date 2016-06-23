use std::collections::HashMap;
use hyper::server::Request as UnwrappedRequest;
use hyper::uri::RequestUri::AbsolutePath;
use hyper::method::Method;
use url::Url;

pub struct Request<'a, 'b: 'a> {
    pub req: UnwrappedRequest<'a, 'b>,
    base_url: String,
    path: String,
    query_params: Option<HashMap<String, String>>
}

impl<'a, 'b> Request<'a, 'b> {
    pub fn wrap_request(req: UnwrappedRequest<'a, 'b>, base_url: &str) -> Request<'a, 'b> {
        let path_with_query = match req.uri {
            AbsolutePath(ref path) => Some(path),
            _ => None
        };
        let full_url = format!("{}{}", base_url, path_with_query.unwrap()).as_str();
        let parsed_url = Url::parse(full_url).unwrap();
        let mut query_params: HashMap<String, String> = HashMap::new();
        Request {
            req: req,
            base_url: base_url.to_string(),
            path: parsed_url.path().to_string(),
            query_params: None
        }
    }

    pub fn path(&self) -> Option<&str> {
        match self.req.uri {
            AbsolutePath(ref path) => Some(path.splitn(2, '?').next().unwrap()),
            _ => None
        }
    }

    pub fn method(&self) -> &Method {
        &self.req.method
    }
}
