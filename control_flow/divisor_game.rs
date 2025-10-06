use std::io;

fn main() {
    let mut x: u32 = loop {
        let mut input: String = String::new();


        println!("Enter the Positive Number : ");
        io::stdin().read_line(&mut input).expect("Failed to read the line");

        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("The entered number is not valid.")
            }
        }
    };

    println!("The Number you entered is : {x}");

    let mut half: u32 = x / 2;
    let mut result: Vec<u32> = Vec::new();

    for i in 1..=half {

        if x % i == 0 {
            result.push(i as u32);
        }
    }

    result.push(x);

    for i in result {
        println!("The Divisors are : {i}");
    }

}
