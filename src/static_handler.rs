
use std::env;
use std::fs;
use super::server::Handler;
use super::http::Response;
use super::http::Request;
use super::http::StatusCode;

pub struct StaticHandler {
    web_root: String
}

impl StaticHandler {
    pub fn new() -> StaticHandler {
        let web_root = match env::var("RUSHTTPD_ROOT") {
            Ok(r) => r,
            Err(_) => format!("{}/public", env!("CARGO_MANIFEST_DIR"))
        };
        println!("Serving files from {}", web_root);
        StaticHandler{web_root}
    }

    fn read_file(&self, fname: &str) -> Option<String> {
        let fpath = format!("{}/{}", self.web_root, fname);
        println!("Request for {}", &fpath);
        match fs::canonicalize(&fpath) {
            Ok(path_buf) =>{
                if !path_buf.starts_with(&self.web_root) {
                    println!("Attempt to use relative paths");
                    return None;
                }
                return fs::read_to_string(fpath).ok();
            },
            Err(_) => None
        }
    }
}


impl Handler for StaticHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.path() {
            "/" => match self.read_file("index.html") {
                Some(content) => Response::new(StatusCode::Ok, Some(content)),
                None => Response::new(StatusCode::NotFound, None)
            },
            "/hello" => match self.read_file("hello.html") {
                Some(content) => Response::new(StatusCode::Ok, Some(content)),
                None => Response::new(StatusCode::NotFound, None)
            },
            _ => match self.read_file(request.path()){
                Some(content) => Response::new(StatusCode::Ok, Some(content)),
                None => Response::new(StatusCode::NotFound, None)
            }
        }
    }
}

