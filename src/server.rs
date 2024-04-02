use std::io::Read;
use std::net::TcpListener;
use std::process;
use std::ptr::read;
use super::Request;
use std::convert::TryFrom;
use std::convert::TryInto;

/// A sample HTTP request looks like this:
/// ```
/// GET / HTTP/1.1\r\n
/// HEADERS \r\n
/// BODY
/// ```
pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self {
            address,
        }
    }

    pub fn run(self) {
        let (ip, port) = self.address.split_at(self.address.find(":").unwrap());
        let port = &port[1..];

        let listener = match TcpListener::bind(&self.address) {
            Ok(listener) => listener,
            Err(e) => {
                println!("Failed to bind to {} <- {}", self.address, e);
                process::exit(1);
            }
        };
        println!("Listening on {} at port {}", ip, port);

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(read_bytes) => {
                            println!("Read {} bytes", read_bytes);
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]){
                                Ok(request) => {},
                                Err(e) => println!("Failed to parse a request: {e}")
                            };
                            let result: &Result<Request, _> = &buffer[..].try_into();
                        }
                        Err(error) => {
                            println!("Failed to read: {}", error);
                            process::exit(1);
                        }
                    };
                }
                Err(e) => {
                    println!("Failed to establish a connection: {}", e);
                }
            }
        }
    }
}