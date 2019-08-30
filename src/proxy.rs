use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};

use std::str::FromStr;

mod http;
pub use http::{Request};
pub use http::url::Url;

pub struct Response {
    status: usize,
    body: String
}

pub struct Server {
    pub port: usize,
    pub requests: Vec<Request>,
}

impl Server {
    pub fn start(port: usize) {
	let tcp_listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();

	for stream in tcp_listener.incoming() {
	    match stream {
		Ok(mut stream) => {
		    // Handle incoming request for proxy
		    let mut req_buffer = [0u8; 4096];
		    match stream.read(&mut req_buffer) {
			Ok(_) => {
			    let req_str = String::from_utf8_lossy(&req_buffer);

			    let req = Request::from_str(&req_str).unwrap();
			    println!("{:#?}", req);
			},
			Err(e) => println!("Unable to read stream: {}", e)
		    }
		}

		Err(e) => {
		    println!("Error connecting to stream: {}", e);
		}
	    }
	}
    }
}
