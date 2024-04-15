use super::method::{Method, MethodError};
use std::convert::TryFrom;
use std::fmt::{Debug, Display, Formatter};
use std::str;
use std::str::Utf8Error;
use crate::http::query_string::QueryString;

#[derive(Debug)]
pub struct Request<'buffer_lifetime> {
    path: &'buffer_lifetime str,
    query_string: Option<QueryString<'buffer_lifetime>>,
    method: Method,
}

impl<'buffer_lifetime> Request<'buffer_lifetime> {
    pub fn path(&self) -> &str {
        self.path
    }

    pub fn query_string(&self) -> Option<&QueryString<'buffer_lifetime>> {
        self.query_string.as_ref()
    }

    pub fn method(&self) -> &Method {
        &self.method
    }
}

impl<'buffer_lifetime> TryFrom<&'buffer_lifetime [u8]> for Request<'buffer_lifetime> {
    type Error = ParseError;

    //GET /search?name=abc&sort=1 HTTP/1.1\r\n...HEADERS...
    fn try_from(buffer: &'buffer_lifetime [u8]) -> Result<Request<'buffer_lifetime>, Self::Error> {
        let request = str::from_utf8(buffer)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;

        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        return Ok(Self {
            path,
            query_string,
            method,
        });
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (index, request_character) in request.chars().enumerate() {
        if request_character == ' ' || request_character == '\r' {
            return Some((&request[..index], &request[index + 1..]));
        }
    }
    None
}

impl From<MethodError> for ParseError {
    fn from(value: MethodError) -> Self {
        self::ParseError::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            ParseError::InvalidRequest => "Invalid Request",
            ParseError::InvalidEncoding => "Invalid Encoding",
            ParseError::InvalidProtocol => "Invalid Protocol",
            ParseError::InvalidMethod => "Invalid Method",
        }
    }
}
