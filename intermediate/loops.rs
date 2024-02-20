pub fn run(){
    let mut count = 0;

    loop {
        count += 1;
        println!("Number: {}", count);

        if count == 20 {
            break;
        }

    }


    while count <= 100 {
        if count % 15 == 0 {
            println!("fizzbuzz");
        } else if count % 3 == 0 {
            println!("fizz");
        } else if count % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", count);
        }

    }

    // For range

    for number in 1..100 {
        if number % 15 == 0 {
            println!("fizzbuzz"); 
        } else if number % 3 == 0 {
            println!("fizz");
        } else if number % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", number);
        }
    }

    


}