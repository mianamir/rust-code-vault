ub fn run(){
    let can_vote = |age: i32|{
        age >= 18
    };

    println!("Can vote: {}", can_vote(18));
    println!("Can vote: {}", can_vote(17));
    println!("Can vote: {}", can_vote(19));

    let mut sample1: i32 = 10;

    let print_var = ||{
        println!("sample1: {}", sample1);
    };

    print_var();

    // sample1 = 20;

    // print_var();

    let mut sample2: i32 = 10;

    let mut print_var2 = ||{
        println!("sample2: {}", sample2);
    };





}