use std::str::FromStr;
use std::convert::From;

#[derive(Debug)]
pub enum Method {
    GET,
    PUT,
    DELETE,
    POST,
    HEAD
}

impl FromStr for Method {
    type Err = MethodError;
    fn from_str(s: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> { 
        match s {
            "GET" => Ok(Self::GET),
            "PUT" => Ok(Self::PUT),
            "DELETE" => Ok(Self::DELETE),
            "POST" => Ok(Self::POST),
            "HEAD" => Ok(Self::HEAD),
            _ => Err(MethodError::new(String::from(s)))
        }
    }
}

#[derive(Debug)]
pub struct MethodError {
    method_name: String
}

impl MethodError {
    pub fn new(name: String) -> MethodError {
        MethodError{method_name: name}
    }
}
