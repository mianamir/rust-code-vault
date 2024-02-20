pub fn run(){
    /**
     * Owenership: Memory
     * 
     * Heap: When putting data on the heap, you request a 
     * certain amount of space. The OS finds space available 
     * and returns an address for that space called pointer.
     * 
     * 
     * 
     * OWenership RULES
     * 1- Each value in Rust has a variable that's called its owner
     * 2- There can only be one owner at a time
     * 3- When the owner goes out of scope the value will 
     * be dropped / disappears
     *
     */


    let mut str1: String = String::from("Amir");

    // let str2: String = str1.clone();
    
    // let str3: String = print_return_str(str1);

    // println!("str3 = {}", str3);

    change_string(&mut str1);

}


fn print_str(x: String){
    
    println!("A string {}", x);

}


fn print_return_str(x: String) -> String{
    
    println!("A string {}", x);

    x

}

fn change_string(name: &mut String){

    let snowflake = "\u{2744}";
    let snowman = "\u{2603}";
    let snowman_without_snow = "\u{26C4}";
    
    name.push_str(" snow is falling today");

    println!("Message: {} {} {} {}", 
                        name, 
                        snowflake, 
                        snowman, 
                        snowman_without_snow);

}