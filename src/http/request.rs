use std::str;
use super::method::Method;
use std::convert::TryFrom;
use std::io::Error;
use std::fmt::{Display, Debug, Formatter};
use std::str::Utf8Error;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl Request {

}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    //GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(buffer: &[u8]) -> Result<Self, Self::Error> {
        match str::from_utf8(buffer){
            Ok(request) => {}
            Err(_) => return Err(ParseError::InvalidEncoding),
        }
        match str::from_utf8(buffer).or(Err(ParseError::InvalidEncoding)) {
            Ok(request) => {}
            Err(e) => return Err(e)
        }

        let request = str::from_utf8(buffer).or(Err(ParseError::InvalidEncoding))?;
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
    InvalidMethod
}

impl ParseError{
    fn message(&self) -> &str{
        match self {
            ParseError::InvalidRequest => {"Invalid Request"}
            ParseError::InvalidEncoding => {"Invalid Encoding"}
            ParseError::InvalidProtocol => {"Invalid Protocol"}
            ParseError::InvalidMethod => {"Invalid Method"}
        }
    }
}

impl Error for ParseError {

}
