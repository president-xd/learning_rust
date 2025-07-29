use std::io;
use rand::Rng; // Crate to generate random numbers
use std::cmp::Ordering; // Crate to compare the number with respect to its order.

// Cargo.toml Content
// [package]
// name = "guessing_game"
// version = "0.1.0"
// edition = "2024"

// [dependencies]
// rand = "0.9.2"


fn main() {
    println!("################### Guessing Game ###################");

    // Random Number Generation using Rand Crate
    let s_number: u32 = rand::rng().random_range(1..101);

    // Looping for continuous input
    loop {

        // Message to ask the user to input
        println!("Enter a number that you guessed : ");

        // Variable to store the number
        let mut number: String = String::new();

        // Taking input from the user
        io::stdin().read_line(&mut number).expect("Failed to read the user input");

        // Converting number (which was string) into u_guess (that is an integer type)
        /*
            Trim() Function: For Space Removal (White Space)
            Parse() Function: For Parsing the value (In our case, string to integer)
            {
                Another thing is OK and Error Condition
                    - Ok, which means that the input was as desired,
                        it will just move the number into variable
                    - Err, which means that the input was not desired,
                        to handle the program crash, we are asking it to continue.
            }
        */
        let u_guess: u32 = match number.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        /*
            - Match to match the input as it was desired to get.
            - cmp() function to compare the input with the reference variable (value).
            - Ordering (use std::cmp::Ordering), to process the comparison as
                it is lesser (to ask user to again input the number greater than recent input),
                greater (to ask user to again input the number less than recent input)
                or equal (to prompt user that you won the game, break the loop).
        */
        match u_guess.cmp(&s_number){
            Ordering::Less => {
                println!("Too small!");
            },
            Ordering::Greater => {
                println!("Too big!");
            },
            Ordering::Equal => {
                println!("You guessed the number!");
                println!("You Won the Game");
                break;
            }
        }
    }
}
