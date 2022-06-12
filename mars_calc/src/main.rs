fn main() {
    let mut  mars_weight=calculate_weight_on_mars(100.0);
    mars_weight=mars_weight * 1000.0; //convert mars to grams
    println!("Weight on Mars:{}g",mars_weight);//macros:call ending with '!',receives variable number of args
//macros:rust code that writes other rust code
    
}
fn calculate_weight_on_mars(weight:f32) -> f32{
(weight/9.81) * 3.711 
}