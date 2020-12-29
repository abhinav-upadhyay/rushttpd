use std::convert::TryFrom;
use std::convert::From;
use std::str::Utf8Error;
use super::method::{Method, MethodError};
use std::str;
use std::fmt::{Display, Debug, Formatter, Result as FmtResult};
use super::QueryString;

#[derive(Debug)]
pub struct Request<'a> {
    path: &'a str,
    query_string: Option<QueryString<'a>>,
    method: Method
}

impl<'a> Request<'a> {
    pub fn path(&self) -> &str {
        self.path
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }

    pub fn method(&self) -> &Method {
        &self.method
    }
}

impl <'a> TryFrom<&'a [u8]> for Request<'a> {
    type Error = ParseError;

    // We only support parsing the first line of the request body consisting of following format:
    // GET /foo?a=b&c=d HTTP/1.1\r\n...
    fn try_from(value: &'a [u8]) -> Result<Request, Self::Error> {
        let request = str::from_utf8(value)?;
        let (method_name, request) = get_next_token(&request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_token(&request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_token(&request).ok_or(ParseError::InvalidRequest)?;
        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }
        let method : Method = method_name.parse()?;
        let mut query_string = None;
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }
        Ok(Self {
            path,
            query_string,
            method
        })
    }

}

fn get_next_token(s :&str) -> Option<(&str, &str)> {
    for (i, c) in s.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&s[..i], &s[i + 1..]));
        }
    }
    None
}

pub enum ParseError {
    InvalidMethod,
    InvalidEncoding,
    InvalidProtocol,
    InvalidRequest
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}


impl Display for ParseError {
    
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult { 
        match self {
            Self::InvalidEncoding => write!(formatter, "Invalid encoding"),
            Self::InvalidMethod => write!(formatter, "Invalid method"),
            Self::InvalidProtocol => write!(formatter, "Invalid protocol"),
            Self::InvalidRequest => write!(formatter, "Invalid request")
        }
        
    }
}

impl Debug for ParseError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> FmtResult { 
        match self {
            Self::InvalidEncoding => write!(formatter, "Invalid encoding"),
            Self::InvalidMethod => write!(formatter, "Invalid method"),
            Self::InvalidProtocol => write!(formatter, "Invalid protocol"),
            Self::InvalidRequest => write!(formatter, "Invalid request")
        }
        
    }
}