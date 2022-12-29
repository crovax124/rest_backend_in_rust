use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fmt::Result as FmtResult;
use std::str;
use std::str::Utf8Error;

pub struct Request {
    path: String,
    //option is whether none or some value of the Type specified
    query_string: Option<String>,
    method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    //GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buf)?;

    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)>{
    for (i, c) in request.chars().enumerate() {
        if c == ' ' {
            return Some((&request[..i], &request[i+1..]));
        }
    }
    None
}
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}
impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self{
        Self::InvalidEncoding
    }
}
impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl Error for ParseError {}
/*
impl TryFrom<&[u8]> for Request {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let string = String::from("asd");
        string.encrypt();
        value.encrypt();
        todo!()
    }
}

//traits are abstract classes and the for keyword is to extend classes.
trait Encrypt {
    fn encrypt(&self) -> Self;
}
impl Encrypt for String {
    fn encrypt(&self) -> Self {
        todo!()
    }
}
impl  Encrypt for &[u8] {
    fn encrypt(&self) -> Self {
        todo!()
    }
}*/

