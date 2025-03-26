use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
    POST,
    GET,
    OPTIONS,
    PUT,
    DELETE,
    HEAD,
    CONNECT,
    TRACE,
    PATCH,
}

impl FromStr for Method {
    type Err = MethodError;

    // this is a method override on the str struct. When we call parse on a str meant to become a Method type
    // it will call this from_str method passing itself to the method and returning a Method type
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "POST" => Ok(Self::POST),
            "GET" => Ok(Self::GET),
            "OPTIONS" => Ok(Self::OPTIONS),
            "PUT" => Ok(Self::PUT),
            "DELETE" => Ok(Self::DELETE),
            "HEAD" => Ok(Self::HEAD),
            "CONNECT" => Ok(Self::CONNECT),
            "TRACE" => Ok(Self::TRACE),
            "PATCH" => Ok(Self::PATCH),
            _ => Err(MethodError),
        }
    }
}

pub struct MethodError;
