pub fn run(){

    // print to the console 

    println!("Print from other file\n");

    // formatting 

    print!("{} is from {}\n", "James", "Texas");

    // positional Arguments 

    print!("{0} is from {1} likes {0}\n", "Jhon", "Mass");


    print!("");

    // Named Arguments 
    print!("{name} likes to play {activity}\n", 
    name="Henry",
    activity="Baseball"
    );

    // placeholder for debugging

    print!("\n{:?}\n", (12, true, "Testing"));


    // math
    print!("10 + 10 = {} \n", 10 + 10);






}