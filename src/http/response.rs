use std::io::{Write, Result as IoResut};
use super::StatusCode;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Self{status_code, body}
    }

    pub fn send(&self, stream: &mut impl Write) -> IoResut<()> {
        let mut body = "";
        if let Some(s) = &self.body {
            body = s;
        };
        write!(stream, "HTTP 1.1 {} {}\r\n\r\n{}", self.status_code, self.status_code.get_reason_phrase(), body)
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let mut body = "";
        if let Some(s) = &self.body {
            body = s;
        };
        write!(f, "HTTP 1.1 {} {}\r\n\r\n{}", self.status_code, self.status_code.get_reason_phrase(), body)
    }
}