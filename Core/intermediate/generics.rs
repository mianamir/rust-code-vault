/ traits

use std::ops::Add;

fn get_sum_gene<T:Add<Output = T>>(x: T, y: T) -> T{

    return x + y;
}


pub fn run(){
    let sum = get_sum_gene(1,2);
    println!("Sum: {}", sum);
    let sum2 = get_sum_gene(1.0,2.0);
    println!("Sum2: {}", sum2);
    let sum3 = get_sum_gene(1.0,2);
    println!("Sum3: {}", sum3);
    let sum4 = get_sum_gene(1,2.0);
    println!("Sum4: {}", sum4);
    let sum5 = get_sum_gene(1.0,2.0);
    println!("Sum5: {}", sum5);

    let sum6 = get_sum_gene(1.0,2.0);
    println!("Sum6: {}", sum6);

    let sum7 = get_sum_gene(1.0,2.0);
    println!("Sum7: {}", sum7);

    let sum8 = get_sum_gene(1.0,2.0);
    println!("Sum8: {}", sum8);

    let sum9 = get_sum_gene(1.0,2.0);
    println!("Sum9: {}", sum9);

    let sum10 = get_sum_gene(1.0,2.0);
    println!("Sum10: {}", sum10);

    let sum11 = get_sum_gene(1.0,2.0);
    println!("Sum11: {}", sum11);

    let sum12 = get_sum_gene(1.0,2.0);
    println!("Sum12: {}", sum12);

}