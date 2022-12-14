use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Formatter, Display, Result as FmtResult, Debug};
use std::str::Utf8Error;
use std::str;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,

}

// impl Request{
//     fn from_byte_array(buf: &[u8]) -> Result<Self, String>{}
// }

impl TryFrom<&[u8]> for Request{
    type Error = ParseError;

    //GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buf)?;

        match get_next_word(request){
            Some((method, request)) => {},
            None => return  Err(ParseError::InvalidRequest),
        }

        get_next_word(request).ok_or(ParseError::InvalidRequest);

        unimplemented!()
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {

    for (i, c) in request.chars().enumerate() {
        if c == ' ' {
            return Some((&request[..i], &request[(i+1)..]))
        }
    }

    None
}

pub enum ParseError{
    InvalidRequest,
    InvalidEnconding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str{
        match self {
            Self::InvalidRequest=>"Invalid Request",
            Self::InvalidEnconding=>"Invalid Enconding",
            Self::InvalidProtocol=>"Invalid Protocol",
            Self::InvalidMethod=>"Invalid Method",
        }
    }
}

impl From<Utf8Error> for ParseError{
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEnconding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}