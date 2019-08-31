use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};

use std::thread;

use std::sync::mpsc::Sender;

use std::str::FromStr;

mod http;
pub use http::{Request};
pub use http::url::Url;

extern crate dns_lookup;
use dns_lookup::lookup_host;

pub struct Response {
    status: usize,
    body: String
}

pub struct Server {
    pub port: usize,
    pub tx: Sender<Request>
}

fn parse_host(host: &str) -> (String, String) {
    // Parse localhost:8080 to ["localhost", "8080"]
    let host_split: Vec<&str> = host.split(":").collect();

    // If port is unknown, default to 80
    if host_split.len() == 2 {
	(String::from(host_split[0]), String::from(":".to_owned() + host_split[1]))
    } else {
	(String::from(host), String::from(":80"))
    }
}

impl Server {
    pub fn new(port: usize, tx: Sender<Request>) -> Server {
	Server {
	    port,
	    tx
	}
    }
    
    pub fn start(&self) {
	// Start the TcpListener for proxy connections
	let tcp_listener = TcpListener::bind(format!("127.0.0.1:{}", self.port)).unwrap();

	for stream in tcp_listener.incoming() {
	    // Clone the sender as to keep from losing the value
	    let tx = self.tx.clone();
	    thread::spawn(move || {
		match stream {
		    Ok(mut stream) => {
			// Handle incoming request for proxy
			let mut req_buffer = [0u8; 4096];
			match stream.read(&mut req_buffer) {
			    Ok(_) => {
				// Parse the http request as a human readable string
				let req_str = String::from_utf8_lossy(&req_buffer[0..]);

				// Create and send the request object to the channel
				let req = Request::from_str(&req_str).unwrap();
				let _ = tx.send(req.clone());

				// Give up if the host is unknown
				if !req.headers.contains_key("Host") {
				    return;
				}

				// Get the host and the port for the DNS query
				let (host, port) = parse_host(req.headers.get("Host").unwrap());
				
				match lookup_host(&host) {
				    Ok(host) => {
					// Create a connection to the requested host
					let mut conn = TcpStream::connect(host[0].to_string() + &port).unwrap();

					// Write the http request to the source host
					conn.write(&req_buffer).unwrap();

					// Read the data from the host and send it to the client (Breaks with Proxy-connection: keep-alive)
					let mut buf = vec![];
					conn.read(&mut buf).unwrap();
					stream.write(&buf).unwrap();
				    }
				    
				    Err(e) => {
					// TODO: Somehow 404
					println!("Error making DNS query: {}", e);
				    }
				}
			    },
			    Err(e) => println!("Unable to read stream: {}", e)
			}
		    }
		    
		    Err(e) => {
			println!("Error connecting to stream: {}", e);
		    }
		}
	    });
	}
    }
}
