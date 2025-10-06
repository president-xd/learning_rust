use std::io;

fn multiply_with_early_exit(n: u64){

    for i in 1..=10 {
        let product: usize = (i * n) as usize;

        if i * n == 42 {
            break;
        }

        println!("{n} X {i} = {product}");
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

    multiply_with_early_exit(n);
}