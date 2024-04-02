mod server;
mod http;

use server::Server;
use http::Request;
use http::Method;

fn main() {
    let server = Server::new(String::from("127.0.0.1:3000"));
    server.run();
}