fn main() {
    //instance of server
    let server= Server::new("127.0.0.1.8080");
    server.run();
}
//struct defination
struct Server{
    addr:String,
}