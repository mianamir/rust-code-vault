se std::thread;
use std::time::Duration;


pub fn run(){
    let thread1 = thread::spawn(|| {
        for i in 1..15 {
            println!("Spawnen thread: {}", i);
            thread::sleep(Duration::from_millis(1));
            
        }
    });

    for i in 1..10 {
        println!("Main thread! {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    thread1.join().unwrap();


}