// ============================================================
//                   Required Libraries / Crates
// ============================================================

use std::io; // For handling user input from terminal
use rand::Rng; // `rand` crate for generating random numbers
use std::cmp::Ordering; // To compare two values and handle match results

// ------------------------------------------------------------
// Cargo.toml Content for Reference:
// [package]
// name = "guessing_game"
// version = "0.1.0"
// edition = "2024"
//
// [dependencies]
// rand = "0.9.2"
// ------------------------------------------------------------

fn main() {
    // ========================================================
    //                 Program Introduction
    // ========================================================
    println!("################### Chapter 2 ###################");
    println!("-------------------------------------------------");
    println!("Hello, Rustaceans!");
    println!("We will learn how to create a simple Guessing Game in Rust.");
    println!("Let's get started!");
    println!("-------------------------------------------------");
    println!("Website Link: https://www.president-xd.com/blog/programming_gussing_game_in_rust");
    println!("-------------------------------------------------\n");

    // ========================================================
    //       Step 1: Generate a Random Number to Guess
    // ========================================================

    // Generate a secret number between 1 and 100 (inclusive).
    // This uses the `rand` crate's random number generator.
    let s_number: u32 = rand::thread_rng().gen_range(1..=100);

    // ========================================================
    //       Step 2: Main Game Loop (User Interaction)
    // ========================================================

    loop {
        // Prompt message for the user
        println!("Enter a number that you guessed:");

        // Step 2a: Read user input into a mutable string
        let mut number: String = String::new();

        // Read the line from standard input
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read the user input");

        // Step 2b: Convert the input (String) into a 32-bit unsigned integer (u32)
        //
        // Use `.trim()` to remove whitespace/newline characters
        // Use `.parse()` to attempt converting it to a number
        //
        // Since `parse()` returns a Result, use `match` to handle both:
        // - `Ok`: successfully parsed number, bind to `u_guess`
        // - `Err`: input wasn't a valid number, prompt again
        let u_guess: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a valid number!");
                continue; // Skip rest of loop and prompt user again
            }
        };

        // Step 2c: Compare user guess with the secret number
        //
        // `cmp()` returns an `Ordering` enum: Less, Greater, Equal
        // `match` is used to handle each case accordingly
        match u_guess.cmp(&s_number) {
            Ordering::Less => {
                println!("Too small! Try a bigger number.");
            },
            Ordering::Greater => {
                println!("Too big! Try a smaller number.");
            },
            Ordering::Equal => {
                println!("🎉 You guessed the number correctly!");
                println!("🏆 You Won the Game!");
                break; // Exit the loop and end the game
            }
        }
    }

    // Game ends once the correct number is guessed
}
