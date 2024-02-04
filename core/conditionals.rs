/// Demonstrates working with conditionals in Rust.
pub fn run() {
    let age: i8 = 45;

    // Check if age is greater than or equal to 34
    let is_of_age = if age >= 34 { true } else { false };

    println!("Is of age: {}", is_of_age);
}
