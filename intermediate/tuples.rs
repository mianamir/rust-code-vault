pub fn run(){

    let person: (&str, &str, i8) = ("Brad", "Mass", 67);

    print!(
        "{} is from {} and is {}\n",
        person.0, person.1, person.2
    )
}