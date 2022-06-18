use std::str::Utf8Error;
use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Formatter,Debug,Result as FmtResult};
use std::str;

 pub struct Request{
    path:String,
    query_string:Option<String>,
    method:Method,

  }
  
  //trait for type
  impl TryFrom<&[u8]> for Request{
    type Error=ParseError;

    //GET /search?name=abc&sort=1 HTTP/1.1

    fn try_from(buf: &[u8])->Result<Self,Self::Error>{
      
       let request =str::from_utf8(buf)?;

       
       //trasform option to result
       //variable shadowing:reusing local variable names ,its not reassigning
        let (method,request)= get_next_word(request).ok_or(ParseError::InvaldEncoding)?;

       unimplemented!()// macro caled on unimplwnrted function to suoprese errors at compile time. once functuion runs errors apperar,
    }
  }

//helper function to get next word
fn get_next_word(request:&str)->Option<(&str,&str)>{

  for (i,c) in request.chars().enumerate(){//char is AN ITERATOR,.enumerate gets the index
if c ==' '{
  return Some((&request[..i],&request[i +1..]))
}
}
None
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
//implentation for from utf8 error
impl From <Utf8Error> for ParseError{
  fn from(_:Utf8Error)->Self{
    Self::InvaldEncoding
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