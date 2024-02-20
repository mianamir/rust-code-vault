pub fn run(){
    let mut arr_it = [11,22,33,44];

    for val in arr_it.iter(){
        print!("{}, ", val);
    }

    println!("");
}

file_handling_with_error_handling.rs
use std::fs::File;
use std::io::{BufReader, ErrorKind};
use std::io::{self, Read};


pub fn run(){
    let path = "lines.txt";

    let output = File::create(path);

    let mut output = match output{
        ok(file) => file,
        Err(error) => {
            panic!("Problem creating the file: {:?}", 
            error)
        },
    };

    write!(output, "Hello, world\n").expect("Failed to write to file");
    write!(output, "Bye-bye\n").expect("Failed to write to file");

    let input = File::open(path).unwrap();

    let buffered = BufReader::new(input);

    for line in buffered.lines(){
        println!("{}", line.unwrap());
    }  

    let output2 = File::create("rand.txt");

    let mut output2 = match output2{
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("rand.txt"){
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            },
        }
    };


}