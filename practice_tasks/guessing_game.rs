use std::io;
use rand::Rng; // Crate to generate random numbers

fn main() {
    println!("################### Guessing Game ###################");

    println!("------------------Guess a number------------------");

    let secret_number = rand::rng().random_range(1..100);

    println!("Enter your guessed number : ");

    let mut in_string: String = String::new();
    io::stdin().read_line(&mut in_string).expect("Failed to read the user from input");

    if (in_string.trim() == secret_number.to_string()){
        println!("The secret number is correct");
        println!("You Won the Game");
        return;
    } else {
        println!("The secret number is incorrect");
        println!("You Lost the Game");
        return;
    }

}
