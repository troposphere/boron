use hyper::server::{Server, Request, Response, Listening};

pub struct Tungsten {
    server: Option<Listening>
}

impl Tungsten {
    pub fn new() -> Tungsten {
        Tungsten { server: None }
    }

    pub fn listen(&mut self, host_port: &str) {
        let server = Server::http(host_port)
            .unwrap()
            .handle(|_req: Request, res: Response| {
                res.send(b"Hello World!").unwrap();
            })
            .unwrap();
        self.server = Some(server);
    }
}
