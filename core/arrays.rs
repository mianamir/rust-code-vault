/// Demonstrates working with arrays in Rust.
pub fn run() {
    // Initialize and manipulate an array
    let mut numbers: [i32; 5] = [11, 22, 33, 44, 55];
    println!("The numbers array contains: {:?}\n", numbers);

    // Modify array elements
    numbers[2] = 222;
    println!("The numbers array contains: {:?}\n", numbers);

    // Access individual elements
    for i in 0..5 {
        println!("The {} number is: {}", i + 1, numbers[i]);
    }

    // Array length and memory usage
    println!("The length of the array is: {}", numbers.len());
    println!("The array occupies {} bytes", std::mem::size_of_val(&numbers));

    // Slice and iteration
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Mutable iteration
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Doubled Numbers Vec: {:?}", numbers);

    // Working with Vectors
    let mut numbers_vec: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("Numbers Vec: {:?}", numbers_vec);

    // Vector operations
    numbers_vec.push(6);
    println!("Numbers Vec: {:?}", numbers_vec);

    numbers_vec.pop();
    println!("Numbers Vec: {:?}", numbers_vec);
}
