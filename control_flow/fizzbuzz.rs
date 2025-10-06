use std::io;

fn fuzz_buzz(n: u64) {
    if n % 3 == 0 && n % 5 == 0 {
        println!("FizzBuzz");
    } else if n % 3 == 0 {
        println!("Fizz");
    } else if n % 5 == 0 {
        println!("Buzz");
    } else {
        println!("{n}");
    }
}

fn main() {

    let mut n: u64 = loop {
        let mut input: String = String::new();

        println!("Enter the number : ");
        io::stdin().read_line(&mut input).expect("Failed to read the line");

        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Invalid Number! Try Again..");
            }
        }
    };

    fuzz_buzz(n);
}