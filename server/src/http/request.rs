use super::method::Method;
use std::convert::TryFrom;

 pub struct Request{
    path:String,
    query_string:Option<String>,
    method:Method,
  }
  impl Request {
    fn from_byte_array(buf:&[u8])->Result<Self,String>{}
  }
  //trait for type
  impl TryFrom<&[u8]> for Request{
    type Error=String;
    fn try_from(value: &[u8])->Result<Self,Self::Error>{
       unimplemented!()// macro caled on unimplwnrted function to suoprese errors at compile time. once functuion runs errors apperar,
    }
  }

  //trait for encryption
  trait Encrypt{
    fn encrypt( &self)->Self;
  }