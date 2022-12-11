use std::io;

fn main() {
    println!("Enter your weight(kg): "); 
    let mut input = String :: new();

    io::stdin().read_line(&mut input).unwrap();

    let weight: f32 = input.trim().parse().unwrap();
    let mars_weight: f32 = calculate_weight_on_mars(weight);

    println!("Weight On Mars: {}kg", mars_weight);
}

fn calculate_weight_on_mars(weight : f32) -> f32 {
    let gravity_on_mars: f32 = 9.81;
    let velocity_on_mars: f32 = 3.711;
    return (weight / gravity_on_mars) * velocity_on_mars;
}