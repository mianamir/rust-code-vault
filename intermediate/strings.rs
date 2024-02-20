pub fn run(){

    let mut hello = String::from("Hello");

    print!("{} world", hello);

    println!("Length: {}", hello.len());


    hello.push_str(" world");

    println!("Updated String : {}", hello);

    // capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // check if empty
    println!("Is Empty: {}", hello.is_empty());

    // contains
    println!("Contains 'World' {}", hello.contains("World"));

    // replace
    println!("Replace: {}", hello.replace("World", "There"));

    // loop through string by whitespace
    for word in hello.split_whitespace(){
        println!("{}", word);
    }

    // create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", hello);


    assert_eq!(12, hello.len());
    assert_eq!(21, hello.capacity());

    println!("{}", hello);


    assert_eq!(12, hello.len());
    assert_eq!(21, hello.capacity());


    
}
