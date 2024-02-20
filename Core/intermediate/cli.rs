use std::env;


pub fn run(){

    let args: Vec<String> = env::args().collect();

    let command = args[1].clone();

    // print!("Args: {:?}\n", args[1]);

    let status = "100%";

    if command == "rust_core" {
        println!("Hi {}, how are you?\n", args[2]);
    } else if command == "status" {
        println!("Status is {}.\n", status);
    }

    println!("Command: {}", command);

}