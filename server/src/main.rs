fn main() {
//Request variables
let get =Method::GET;
let delete=Method::DELETE;
let post =Method::POST;
let put=Method::PUT;
    
    //instance of server
    let server= server::Server::new("127.0.0.1:8080".to_string());//convert from string literal/slice to String
    server.run();
}
mod server {


//struct defination
 pub struct Server{
    addr:String,
}
//struct implementation
 impl Server{
  // struc have 2 functionalities:1method or 2.associated function/static function e.g new() 
  //methods : always take 1st parameter of self(instance of struc method is bn called)
  //:: to acess associated functions
 pub fn new(addr:String)->Self{
    Self{
        addr
    }
  }
  //run method
 pub fn run(self){// run takes ownership of struc instance
    println!("Server  Listening on {}" ,self.addr);
  }
}

}
mod http{
//request
mod request{
struct Request{
    path:String,
    query_string:Option<String>,
    method:Method,
  }
}
//enum
mod method{
enum Method{
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