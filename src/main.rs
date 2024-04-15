#![allow(dead_code)]
#![allow(unused_imports)]

mod server;
mod http;
mod website_handler;

use server::Server;
use http::Request;
use website_handler::WebsiteHandler;
use std::env;
//use http::Method;

fn main() {
    let default_path = format!("{}\\public", env!("CARGO_MANIFEST_DIR").replace("/", "\\"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("public path is {}", public_path);
    let server = Server::new(String::from("127.0.0.1:3000"));
    server.run(WebsiteHandler::new(public_path));
}