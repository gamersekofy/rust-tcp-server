# Introduction

This repository contains a small and simple server I made as part of [this course on Udemy](https://www.udemy.com/course/rust-fundamentals/?couponCode=ST20MT50724).

The server is a simple HTTP server that listens on port 3000 and responds with a simple HTML page. The objective of this course was to learn the basics of Rust, so the server is very simple and does not have any advanced features.

# How to run

To run the server, you need to have Rust installed. You can install Rust by following the instructions on the [official website](https://www.rust-lang.org/tools/install).

Once you have Rust installed, you can run the server by executing the following command:

```bash
cargo run
```

This will compile the server and start it. You can then access the server by opening a web browser and navigating to `http://localhost:3000`.

## Modifying the server

To change the port the server listens on, you can modify the `main.rs` file and change the following line:

```rust
fn main() {
    /*...*/
    let server = Server::new(String::from("127.0.0.1:3000"));
    /*...*/
}
```

You can replace `3000` with any other port number you want.