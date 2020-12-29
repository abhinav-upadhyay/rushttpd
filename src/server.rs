use std::io::{Read};
use std::net::TcpListener;
use std::convert::TryFrom;
use super::http::{Request, Response, StatusCode, ParseError};

#[derive(Debug)]
pub struct Server {
    addr: String,
}

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, _: &ParseError) -> Response {
        Response::new(StatusCode::BadRequest, Some("Bad request".to_string()))
    }
}

impl Server {
    pub fn new(addr: String) -> Server {
        Server{addr}
    }

    pub fn run(&self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream, _)) =>  {
                    let mut buf = [0; 1024];
                    match stream.read(&mut buf) {
                        Ok(_) => {
                            println!("{}", String::from_utf8_lossy(&buf));
                            let response = match Request::try_from(&buf[..]) {
                                Ok(request) => {
                                    handler.handle_request(&request)
                                },
                                Err(e) => {
                                    handler.handle_bad_request(&e)
                                }
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("response send failed: {}", e);
                            }
                        },
                        Err(e) => println!("Request read failed: {}", e)
                    }
                },
                Err(e) => println!("Failed to establish connection: {}", e)
            }
        }
    }
}
