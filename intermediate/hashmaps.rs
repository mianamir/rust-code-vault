use std::collections::HashMap;

pub fn run(){
    
    let mut heroes = HashMap::new();

    heroes.insert(String::from("Superman"), String::from("Clark Kent"));

    heroes.insert(String::from("Batman"), String::from("Bruce Wayne"));

    heroes.insert(String::from("Wonder Woman"), String::from("Diana Prince"));

    heroes.insert(String::from("Flash"), String::from("Barry Allen"));

    heroes.insert(String::from("Green Lantern"), String::from("Hal Jordan"));

    heroes.insert(String::from("Green Arrow"), String::from("Oliver Queen"));

    heroes.insert(String::from("Aquaman"), String::from("Arthur Curry"));

    heroes.insert(String::from("Cyborg"), String::from("Victor Stone"));

    heroes.insert(String::from("Shazam"), String::from("Billy Batson"));


    for (key, value) in &heroes {
        println!("{}: {}", key, value);
    }

    println!("");

    println!("Length: {}", heroes.len());

    if heroes.contains_key("Batman"){
        let the_batman = heroes.get("Batman");

        match the_batman {
            Some(x) => println!("Batman: {:?}", the_batman),
            None => println!("Batman not found"),
        }
    }


}
