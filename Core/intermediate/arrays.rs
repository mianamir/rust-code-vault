pub fn run(){

    let mut numbers: [i32; 5] = [11, 22, 33, 44, 55];


    println!("The numbers array contains: {:?}\n", numbers);

    numbers[2] = 222;

    println!("The numbers array contains: {:?}\n", numbers);

    println!("The first number is: {}", numbers[0]);

    println!("The second number is: {}", numbers[1]);

    println!("The third number is: {}", numbers[2]);

    println!("The fourth number is: {}", numbers[3]);

    println!("The fifth number is: {}", numbers[4]);

    println!("The length of the array is: {}", numbers.len());

    println!("The array occupies {} bytes", std::mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers[0..2];

    println!("Slice: {:?}", slice);

    for x in numbers.iter(){
        println!("Number: {}", x);
    }

    for x in numbers.iter_mut(){
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);

    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    println!("Numbers Vec: {:?}", numbers);

    numbers.push(6);

    println!("Numbers Vec: {:?}", numbers);

    numbers.pop();

    println!("Numbers Vec: {:?}", numbers);

    println!("The first number is: {}", numbers[0]);

    println!("The second number is: {}", numbers[1]);

    println!("The third number is: {}", numbers[2]);

    println!("The fourth number is: {}", numbers[3]);

    println!("The fifth number is: {}", numbers[4]);

    println!("The length of the array is: {}", numbers.len());

    println!("The array occupies {} bytes", std::mem::size_of_val(&numbers));


}