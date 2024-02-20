pub fn run(){

    let mut numbers: Vec<i32> = vec![11, 22, 33, 44, 55];



    // sort the vector
    numbers.sort();

    // iterate over the sorted vector
    for n in &numbers {
        println!("{}", n);
    }

    // push to the vector
    numbers.push(66);

    // pop from the vector
    numbers.pop();

    // get single value
    println!("Single value: {}", numbers[0]);

    // get vector length
    println!("Vector length: {}", numbers.len());

    // vector are stack allocated
    println!("Vector occupies {} bytes", std::mem::size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    // loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers Vec: {:?}", numbers);


    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers Vec: {:?}", numbers);


    // loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers Vec: {:?}", numbers);

    // loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers Vec: {:?}", numbers);

    

}
