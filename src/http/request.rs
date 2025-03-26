use super::method::{Method, MethodError};
use super::QueryString;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str, // must specify lifetime for reference inside struct
    query_string: Option<QueryString<'buf>>,
    method: Method,
    headers: Headers
}

impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        &self.path
    }
    pub fn method(&self) -> &Method {
        &self.method
    }
    pub fn query_string(&self) -> Option<&QueryString<'buf>> {
        self.query_string.as_ref()
    }
}


// TryFrom is used when the conversion can fail
//   👇🏻 - lifetime identifier
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParserError;

    // GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error> {
        // convert u8 buffer array to utf8 encoded string
        let request = str::from_utf8(buf)?;

        // gets the first chunk from a request, ends up as ("GET", ".. rest")
        let (method, request) = get_next_word(request).ok_or(ParserError::InvalidMethod)?; // GET
        
        // gets the first chunk from ".. rest", ends up as ("/search?name=abc&sort=1", ".. rest")
        let (mut path, request) = get_next_word(request).ok_or(ParserError::InvalidMethod)?; // /search?name=abc&sort=1
        
        // gets the first chunk from ".. rest", ends up as ("HTTP/1.1", ".. rest")
        let (protocol, headers) = get_next_word(request).ok_or(ParserError::InvalidMethod)?; // HTTP/1.1

        // we only handle this protocol
        if protocol != "HTTP/1.1" {
            return Err(ParserError::InvalidProtocol);
        }

        // goto Method.rs - to see how this works
        let method: Method = method.parse()?;

        let mut query_string = None;
        // get the byte index from path of the "?" and grab all bytes from that position to end of path to get all queries
        // also grab the path from pre "?"
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..])); // adds 1 byte to i - not one character
            path = &path[..i];
        }

        // returns an instance of Request with all of the necessary information
        Ok(Self {
            path,
            query_string,
            method,
            headers: Headers::new(headers.to_string())
        })
    }
}

// read the request string slice up to a space or return and return a slice of the pre space and the post space
fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (index, char) in request.chars().enumerate() {
        if char == ' ' || char == '\r' {
            return Some((&request[..index], &request[index + 1..]));
        }
    }
    None
}

pub enum ParserError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParserError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid request",
            Self::InvalidEncoding => "Invalid encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl Error for ParserError {
    // by implimenting this ourselves itll force us to meet some expectations for our error types
}

impl From<MethodError> for ParserError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParserError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParserError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

#[derive(Debug)]
struct Headers {
    headers_map: HashMap<String, String>
}

impl Headers{
    fn new(headers: String) -> Self{
        let mut map: HashMap<String, String> = HashMap::new();
        let split_headers: Vec<&str> = headers.split("\r\n").collect();
        for header in split_headers.iter(){
            let split_header: Vec<&str> = header.split(": ").collect();
            if split_header.len() < 2 {
                continue;
            }
            let key = split_header[0];
            let value = split_header[1];
            map.insert(key.to_owned(), value.to_owned());
        }
        Self { headers_map: map }
    }
}
