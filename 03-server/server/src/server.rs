use crate::http::Request;
use std::convert::TryForm;
use std::net::TcpListener;
use std::io::{Read, Write};

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { address: addr }
    }

    pub fn run(&mut self) {
        println!("Listening to {}", self.address);
        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    println!("Client connected!");

                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received Request: {}", String::from_utf8_lossy(&buffer));
                            // Request::try_form(&buffer as &[u8]) only contain values
                            Request::try_form(&buffer[..]) // used slice to only get all the value 
                            let res: &Result<Request,_> = &buffer[..].try_into();
                            match res {
                                Ok(request) => {
                                    println!("Parsed Request: {:?}", request);
                                    // Here i handle the request and send a response
                                    let response = "HTTP/1.1 200 OK\r\n\r\nHello, World!";
                                    stream.write_all(response.as_bytes()).unwrap();
                                }
                                Err(e) => println!("Failed to parse request: {}", e),
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("Connection failed: {}", e),
            }
        }
    }
}
