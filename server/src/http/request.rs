use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Formatter,Debug,Result as FmtResult};

 pub struct Request{
    path:String,
    query_string:Option<String>,
    method:Method,

  }
  
  //trait for type
  impl TryFrom<&[u8]> for Request{
    type Error=ParseError;

    //GET /search?name=abc&sort=1 HTTP/1.1

    fn try_from(value: &[u8])->Result<Self,Self::Error>{
       unimplemented!()// macro caled on unimplwnrted function to suoprese errors at compile time. once functuion runs errors apperar,
    }
  }


  //Error enum
 pub enum ParseError{
InvalidRequest,//broken request
InvaldEncoding,
InvalidProtocal,
InvalidMethod,

  }
  impl ParseError{
    fn message(&self)->&str{
      match self{
        Self::InvalidRequest=>"Invalid Request",
       Self:: InvaldEncoding=>"Invalid Encoding",
        Self::InvalidProtocal=>"Invalid Protocal",
         Self::InvalidMethod=>"Invalid Method",
      }
    }
  }
  //impl for display 
  impl Display for ParseError{
    fn fmt(&self,f:&mut Formatter)->FmtResult{
      write!(f," {}",self.message())
    }
  }
  
  //debug trait
  impl Debug for ParseError{
    fn fmt(&self,f:&mut Formatter)->FmtResult{
      write!(f," {}",self.message())
    }
  }


impl Error for ParseError{}