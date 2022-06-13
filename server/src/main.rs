use http::request::Request;
use server::Server;
mod server;
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


/*
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/ 