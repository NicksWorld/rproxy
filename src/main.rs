mod proxy;
use std::sync::mpsc;

use std::thread;

use proxy::{Server, Request};

fn main() {
    // Create channel for communicating http requests
    let (tx, rx) = mpsc::channel();

    // Create and run the proxy server in its own thread
    let proxy_server = Server::new(2020, tx);
    thread::spawn(move || {
	proxy_server.start();
    });

    // Recive requests from the async channel
    loop {
	for req in rx.recv() {
	    println!("{:#?}", req);
	}
    }
}
