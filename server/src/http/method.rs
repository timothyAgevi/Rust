//crate to convert str to enum
use std::str::FromStr

//enum

pub enum Method{
    GET,
    DELETE,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}
impl FromStr for Method{
    type Err=MethodError;
    fn from_str(s:&str)->Result<Self,self::Err>{
        match s{
            "GET"=>Ok(Self::GET),
    "DELETE"=>Ok(Self::DELETE),
    "POST"=>Ok(Self::POST),
    "PUT"=>Ok(Self::PUT),
    "HEAD"=>Ok(Self::HEAD),
    "CONNECT"=>Ok(Self::GET),
    "OPTIONS"=>Ok(Self::OPTIONS),
    "TRACE"=>Ok(Self::TRACE),
    "PATCH"=>Ok(Self::PATCH),
    _=>Err(MethodError::from("error")),
        }
    }
 }
 //make MethodError public
 pub struct MethodError;