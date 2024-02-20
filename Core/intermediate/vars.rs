pub fn run(){
    /*
    Data Types
     */

    let name = "Hohn";

    let mut age = 45;

    println!("My age is {}\n", age);

    age = 46;
    

    println!("My name is {} and age is {}\n", name, age);


    // define constant 

    const ID: i32 = 001;

    println!("ID: {}\n", ID);

    // Assign multiple vars

    let (my_name, my_age) = ("Hohn", 45);

    println!(
        "My name is {} and age is {}\n",
        my_name, my_age)


}