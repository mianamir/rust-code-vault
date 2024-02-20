/ Structs are used to create custom data types 
struct Color{
    red: u8,
    green: u8,
    blue: u8

}

// Tuple Struct
// struct Color(u8, u8, u8);

struct Person{
    first_name: String,
    last_name: String
}

impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    // Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }  


}

pub fn run(){

    let mut c = Color{
        red: 255,
        green: 0,
        blue: 0
    };

    
    println!("Color: {} {} {}", c.red, c.green, c.blue);

    c.red = 200;

    println!("Color: {} {} {}", c.red, c.green, c.blue);

    // Use Person struct 

    let mut p = Person::new("John", "Doe");

    println!("Person: {} {}", p.first_name, p.last_name);

    p.first_name = String::from("John");

    println!("Person: {} {}", p.first_name, p.last_name);

    println!("Person: {}", p.full_name());

    p.set_last_name("Doe");

    

    println!("Person: {:?}", p.to_tuple());



}

pub fn run2(){
    
    // create a new struct called Customer

    // struct Customer{
    //     name: String,
    //     address: String,
    //     balance: f32,    
    // }

    // let mut bob = Customer{
    //     name: String::from("Bob Smith"),
    //     address: String::from("123 Main St"),
    //     balance: 100.0,
    // };

    // bob.address = String::from("456 Main St");

    // println!("Bob's address is {}", bob.address);

    // struct Rectangle<T, U>{
    //     length: T,
    //     width: U,
    
    // }

    // let rect1 = Rectangle{
    //     length: 50,
    //     width: 30,
    // };

    // let rect2 = Rectangle{
    //     length: 10.0,
    //     width: 4.0,
    // };

    // println!("The area of the rectangle is {} square pixels.", rect1.area());
    // println!("The area of the rectangle is {} square pixels.", rect2.area());
    
    const PI: f32 = 3.141592;

    trait Shape{
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }

    struct Rectangle{
        length: f32,
        width: f32,
    }

    struct Circle{
        length: f32,
        width: f32,
    }


    impl Shape for Rectangle {
        fn new(length: f32, width: f32) -> Rectangle {
            return Rectangle{length,width};
        }
        fn area(&self) -> f32 {
            return self.length * self.width;
        }
    }

    impl Shape for Circle {
        fn new(length: f32, width: f32) -> Circle{
            return Circle{length,width};
        }

        fn area(&self) -> f32 {
            return (self.length / 2.0 ).powf(2.0) * PI;
        }
    }

    let rect1: Rectangle = Shape::new(50.0, 30.0);
    let circle1: Circle = Shape::new(50.0, 30.0);

    println!("The area of the rectangle is {} square pixels.", rect1.area());
    println!("The area of the circle is {} square pixels.", circle1.area());


    
    
}