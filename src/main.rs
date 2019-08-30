mod proxy;

use proxy::{Server, Request, Response};

fn main() {
    Server::start(2020);
}
