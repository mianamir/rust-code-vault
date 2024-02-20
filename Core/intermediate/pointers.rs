ub fn run(){
    // Reference Pointers - Point to a resournce in memory


    // Create a reference to i32
    let ref1 = &mut 10;

    // Create a reference to a String
    let ref2 = &String::from("Hello");

    // Dereference ref1 and print it
    println!("Deref ref1: {}", *ref1);

    // Dereference ref2 and print it
    println!("Deref ref2: {}", *ref2);

   

}