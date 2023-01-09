use std::net::TcpListener;
use std::io::{Read, Write};
use crate::http::{Request, Response, StatusCode};
use std::convert::TryFrom;


pub struct Server {
    address: String,
}


impl Server {
    pub fn new(address: String) -> Self {
        Self {
            address
        }
    }
    pub fn run(self) {
        println!("Listening on: {}", self.address);
        let listener = TcpListener::bind(&self.address).unwrap();

        'outer: loop {
            //Match is a just a sugar syntax for switch statements
            match listener.accept() {
                Ok((mut stream, address)) => {
                    println!("Connection established on {}: ", address);
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(result) => {
                            println!("Recieved a request: {}", String::from_utf8_lossy(&buffer));


                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                    //  let response = Response::new(StatusCode::NotFound,None);
                                    Response::new(StatusCode::Ok,
                                                  Some("IT WORKS!!!!".to_string()), )
                                }
                                Err(e) => {
                                    println!("Failed to parse a request: {}", e);
                                    Response::new(StatusCode::BadRequest, None)
                                }
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
