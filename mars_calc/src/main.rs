fn main() {
    let mars_weight=calculate_weight_on_mars(100.0);
    println!("Weight on Mars:{}kg",mars_weight);//macros:call ending with '!',receives variable number of args
//macros:rust code that writes other rust code
    
}
fn calculate_weight_on_mars(weight:f32) -> f32{
(weight/9.81) * 3.711 
}