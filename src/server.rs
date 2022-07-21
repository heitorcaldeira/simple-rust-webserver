use crate::http::{ParseError, Request, Response, StatusCode};
use std::io::Read;
use std::net::TcpListener;

pub struct Server<'a> {
    addr: &'a str,
}

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, err: &ParseError) -> Response {
        println!("Failed to parse request: {}", err);
        return Response::new(StatusCode::BadRequest, None);
    }
}

impl<'a> Server<'a> {
    pub fn new(addr: &'a str) -> Self {
        Server { addr }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Err(_) => {
                    println!("Err");
                }
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 2048];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            let response = match Request::try_from(&buffer as &[u8]) {
                                Ok(request) => {
                                    println!("{:?}", request);
                                    handler.handle_request(&request)
                                }
                                Err(err) => {
                                    handler.handle_bad_request(&err)
                                }
                            };
                            if let Err(_) = response.send(&mut stream) {
                                println!("Failed to send response");
                            }
                        }
                        Err(_) => println!("Failed to read"),
                    };
                }
            }
        }
    }
}
