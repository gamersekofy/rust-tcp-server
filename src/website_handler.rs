use std::{fs, path};

use crate::http::{Method, Request, Response, StatusCode};

use super::server::Handler;

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}\\{}", self.public_path, file_path);

        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory Traversal Attack Attempted: {}", file_path);
                    Some("".to_string())
                }
            }
            Err(_) => None,
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/login" => Response::new(StatusCode::Ok, self.read_file("login.html")),
                "/signup" => Response::new(
                    StatusCode::Ok,
                    Some(
                        r#"
                    <h1>Sign Up</h1>
                    <form>
                        <label for="username">Username:</label>
                        <input type="text" id="username" name="username"><br><br>
                        <label for="email">Email:</label>
                        <input type="email" id="email" name="email"><br><br>
                        <label for="password">Password:</label>
                        <input type="password" id="password" name="password"><br><br>
                        <input type="submit" value="Sign Up">
                    </form>
                "#
                        .to_string(),
                    ),
                ),
                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
            Method::DELETE => todo!(),
            Method::POST => todo!(),
            Method::PUT => todo!(),
            Method::HEAD => todo!(),
            Method::CONNECT => todo!(),
            Method::OPTIONS => todo!(),
            Method::TRACE => todo!(),
            Method::PATCH => todo!(),
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
