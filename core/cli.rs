use std::env;

/// Handles command-line arguments and performs actions based on the provided command.
pub fn run() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a command.");
        return;
    }

    let command = args[1].clone();
    let status = "100%";

    if command == "rust_core" {
        println!("Hi {}, how are you?\n", args[2]);
    } else if command == "status" {
        println!("Status is {}.\n", status);
    }

    println!("Command: {}", command);
}
