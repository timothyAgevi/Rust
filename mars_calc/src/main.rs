use std::io ;

fn main() {
    //allocate new string
    let mut input=String::new();//string stored in heap dsince value unkoen at compile time,inputis its pointer
    io::stdin().read_line(&mut input).unwrap();
    println!("Input :{}",input);
    let  mars_weight=calculate_weight_on_mars(100.0);
    // mars_weight= mars_weight * 1000.0; //convert mars to grams
    println!("Weight on Mars:{}kg",mars_weight);//macros:call ending with '!',receives variable number of args
//macros:rust code that writes other rust code
    
}
fn calculate_weight_on_mars(weight:f32) -> f32{
(weight/9.81) * 3.711 
}
//Owenership

//1. Each value in Rust is owened by a variable

//2.When the owner goes out of scope,the value will be dealloated.

//3.there can only be 1 owner at a time.

