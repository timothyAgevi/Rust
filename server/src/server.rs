use std::net::{TcpListener, TcpStream};
use std::io::Read;
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

    let listener = TcpListener::bind(&self.addr).unwrap();
    loop{
//matching expression
match listener.accept(){
  Ok(( mut stream, _) )=>{
 let mut buffer =[ 0;1024];
  match stream.read(&mut buffer){
    Ok(_)=>{
      println!("Received a request: {}",String::from_utf8_lossy(&buffer));//haadled wen validating utf8
    }
    Err(e)=>print!("Failed to read from connection :{}",e)
  }
}
Err(e)=>print!("Failed to establish a connection: {}",e)
     }
    
    }
      
  }
}