use super::method::Methods;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Debug;
use std::str;
// optional as it is already imported by compiler
// pub enum Option<T> {
//     None, Some(T)
// }

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Methods,
}

impl TypeForm<&[u8]> for Request {
    type Error = String;

    // GET /search?name=rust HTTP/1.1
    fn try_form(buf: &[u8]) -> Result<Self, Self::Error> {
        match str::from_utf8(buff) {
            Ok(request_str) => {},
            Err(_) => return Err(ParsedError::InvalidEncoding.message()),
        }

        match str::from_utf8(buf).or(Err(ParsedError::InvalidEncoding.message())) {
            
        }
    }
}

impl Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Debug for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

pub enum ParsedError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParsedError {
    pub fn message(&slef) -> String {
        match self {
            ParsedError::InvalidRequest => "Invalid request format".to_string(),
            ParsedError::InvalidEncoding => "Invalid encoding".to_string(),
            ParsedError::InvalidProtocol => "Invalid protocol version".to_string(),
            ParsedError::InvalidMethod => "Invalid HTTP method".to_string(),
        }
    }
}

impl Error for ParsedError {
    fn description(&self) -> &str {
        match self {
            ParsedError::InvalidRequest => "Invalid request format",
            ParsedError::InvalidEncoding => "Invalid encoding",
            ParsedError::InvalidProtocol => "Invalid protocol version",
            ParsedError::InvalidMethod => "Invalid HTTP method",
        }
    }
}