use super::method::Method;
use std::convert::TryFrom;

pub struct Request {
    path: String,
    //option is whether none or some value of the Type specified
    query_string: Option<String>,
    method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = String;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        todo!()
    }
}

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

