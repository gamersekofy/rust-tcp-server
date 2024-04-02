use super::method::Method;
use std::convert::TryFrom;
use std::io::Error;
use std::fmt::{Display, Debug};

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl Request {

}

impl TryFrom<&[u8]> for Request {
    type Error = String;

    fn try_from(buffer: &[u8]) -> Result<Self, Self::Error> {
        todo!()
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol
}

impl Error for ParseError {

}
