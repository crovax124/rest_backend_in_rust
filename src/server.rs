use std::net::TcpListener;
use std::io::{Read};
use crate::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse Request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    address: String,
}


impl Server {
    pub fn new(address: String) -> Self {
        Self {
            address
        }
    }
    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on: {}", self.address);
        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            //Match is a just a sugar syntax for switch statements
            match listener.accept() {
                Ok((mut stream, address)) => {
                    println!("Connection established on {}: ", address);
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_result) => {
                            println!("Recieved a request: {}", String::from_utf8_lossy(&buffer));


                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e)
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to parse a request: {}", e)
                            }
                        }
                        Err(e) => println!("Failed to read the Websocket: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}", e),

                // _ => {} default case
            }
        }
    }
}
