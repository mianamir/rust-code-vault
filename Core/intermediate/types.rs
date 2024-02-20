pub fn run(){
    // data types


    // data types
    let my_int: i32 = 5;
    let my_float: f64 = 3.14;
    let my_bool: bool = true;
    let my_char: char = 'a';
    let my_string: String = "Hello".to_string();

    println!("My int is {} and max value can be {}: ", 
            my_int, std::i32::MAX);
    println!("My float is {} and max value can be {}: ", 
              my_float, std::i64::MAX);
    println!("My bool is {}", my_bool);
    println!("My char is {}", my_char);
    println!("My string is {}", my_string);


    // unicodes

    let face = '\u{1F600}';
    println!("My face is {}", face);

    
}
