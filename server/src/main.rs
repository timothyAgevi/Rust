fn main() {
    //instance of server
    let server= Server::new("127.0.0.1.8080");
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
}