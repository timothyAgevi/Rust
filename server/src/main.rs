use http::request::Request;
use server::Server;
fn main() {
//Request variables
// let get =super::method ::Method::GET;
// let delete=Method::DELETE;
// let post =Method::POST;
// let put=Method::PUT;
    
    //instance of server
    let server= Server::new("127.0.0.1:8080".to_string());//convert from string literal/slice to String
    server.run();
}

mod http{
  
//request
 pub mod request{
  use super::method::Method;
 pub struct Request{
    path:String,
    query_string:Option<String>,
    method:Method,
  }
}
//enum
 pub mod method{
 pub enum Method{
    GET,
    DELETE,
    POST,
    PUT,
    // HEAD,
    // CONNECT,
    // OPTIONS,
    // TRACE,
    // PATCH,
}
}
}
/*
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/ 