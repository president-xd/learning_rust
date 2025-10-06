use std::io;

fn collatz(mut n: u64) -> u64 {
    if n == 1 {
        1
    } else if n % 2 == 0 {
        n / 2
    } else {
        3 * n + 1
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

    let n: u64 = collatz(n);
    println!("The number according to collatz rule is : {n} ");
}