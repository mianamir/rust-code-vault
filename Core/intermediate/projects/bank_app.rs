use std::{backtrace, thread};
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};


pub fn run(){
    pub struct Bank{
        balance: f32,
        name: String,
        account_number: i32,
    }

    impl Bank{
        pub fn new(balance: f32, name: String, account_number: i32) -> Bank{
            Bank{
                balance,
                name,
                account_number,
            }
        }

        pub fn deposit(&mut self, amount: f32){
            self.balance += amount;
        }

        pub fn withdraw(&mut self, amount: f32){
            self.balance -= amount;
        }

        pub fn get_balance(&self) -> f32{
            self.balance
        }

        pub fn get_name(&self) -> String{
            self.name.clone()
        }

        pub fn get_account_number(&self) -> i32{
            self.account_number
        }

        pub fn set_name(&mut self, name: String){
            self.name = name;
        }

        pub fn set_account_number(&mut self, account_number: i32){
            self.account_number = account_number;
        }

        pub fn set_balance(&mut self, balance: f32){
            self.balance = balance;
        }

        pub fn transfer(&mut self, amount: f32, other: &mut Bank){
            self.withdraw(amount);
            other.deposit(amount);
        }

        pub fn transfer_to_other_bank(&mut self, amount: f32, other: &mut Bank){
            self.withdraw(amount);
            other.deposit(amount);
        }
    }

    let mut bank1 = Bank::new(100.0, String::from("John"), 123456789);

    let mut bank2 = Bank::new(100.0, String::from("John"), 987654321);

    bank1.transfer(10.0, &mut bank2);

    println!("{} has a balance of {}", bank1.get_name(), bank1.get_balance());

    println!("{} has a balance of {}", bank2.get_name(), bank2.get_balance());

    bank1.transfer_to_other_bank(10.0, &mut bank2);

    println!("{} has a balance of {}", bank1.get_name(), bank1.get_balance());

    println!("{} has a balance of {}", bank2.get_name(), bank2.get_balance());

    bank1.set_name(String::from("John"));

    println!("{} has a balance of {}", bank1.get_name(), bank1.get_balance());

    bank1.set_account_number(987654321);

    println!("{} has a balance of {}", bank1.get_name(), bank1.get_balance());

    bank1.set_balance(100.0);

    println!("{} has a balance of {}", bank1.get_name(), bank1.get_balance());

    bank1.deposit(10.0);

    println!("{} has a balance of {}", bank1.get_name(), bank1.get_balance());

    bank1.withdraw(10.0);

    println!("{} has a balance of {}", bank1.get_name(), bank1.get_balance());

    bank1.transfer(10.0, &mut bank2);

    println!("{} has a balance of {}", bank1.get_name(), bank1.get_balance());

    println!("{} has a balance of {}", bank2.get_name(), bank2.get_balance());

    bank1.transfer_to_other_bank(10.0, &mut bank2);

    println!("{} has a balance of {}", bank1.get_name(), bank1.get_balance());



    fn customer(the_bank: &mut Bank){
        let mut balance = the_bank.get_balance();
        let mut name = the_bank.get_name();
        let mut account_number = the_bank.get_account_number();
    }

    fn employee(the_bank: &mut Bank){
        let mut balance = the_bank.get_balance();
        let mut name = the_bank.get_name();
        let mut account_number = the_bank.get_account_number();
    }

    let mut bank3 = Bank::new(100.0, String::from("John"), 123456789);

    let mut bank4 = Bank::new(100.0, String::from("John"), 987654321);

    let customer_thread = thread::spawn(move || {
        customer(&mut bank3);
    });

    let employee_thread = thread::spawn(move || {
        employee(&mut bank4);
    });

    customer_thread.join().unwrap();
    employee_thread.join().unwrap();

    // println!("{} has a balance of {}", bank3.get_name(), bank3.get_balance());

    // println!("{} has a balance of {}", bank4.get_name(), bank4.get_balance());

    // parallel code

    fn withdraw(the_bank: &Arc<Mutex<Bank>>, amt: f32){
        let mut bank_ref = the_bank.lock().unwrap();

        if bank_ref.balance < 5.00{
            println!("Curent balance is : {} 
            Withdrawal a smaller amount", bank_ref.balance);
        }else{
            bank_ref.balance -= amt;
            println!("Customer withdrew {}
            New balance is {}", amt, bank_ref.balance);

        }
    }

    // fn customer(the_bank: &Arc<Mutex<Bank>>){
    //     withdraw(&the_bank, 5.00);
    // }

    // let backtrace = Arc::
    //                                         new(Mutex::new(Bank
    //                                         ::new(100.0, String::
    //                                             from("John"), 123456789)));


}