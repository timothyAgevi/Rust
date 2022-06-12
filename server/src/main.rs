fn main() {
    //instance of server
    let server= Server::new("127.0.0.1:8080".to_string());//convert from string literal/slice to String
    server.run();
}
//struct defination
struct Server{
    addr:String,
}
//struct implementation
impl Server{
  // struc have 2 functionalities:1method or 2.associated function/static function e.g new() 
  //methods : always take 1st parameter of self(instance of struc method is bn called)
  //:: to acess associated functions
  fn new(addr:String)->Self{
    Self{
        addr
    }
  }
  //run method
  fn run(self){// run takes ownership of struc instance
    println!("Server  Listening on {}" ,self.addr);
  }
}
//request
struct Request{
    path:String,
    query_string:String,
    method:Methods,
}
//enum
enum Methods{
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

/*
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/ 