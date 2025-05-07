// Import standard input/output modules
use std::io::{self, stdin, Write};

fn main() {

    /*********************************************************
     * STRING INPUT
     * -------------------------------------------------------
     * Strings are stored in the `String` type which is
     * growable, UTF-8 encoded, and heap allocated.
     * `read_line()` reads the user input including the newline,
     * so we use `.trim()` to remove it before printing.
     *********************************************************/
    println!("Input Some Strings:");
    let mut string_in = String::new(); // Create an empty String buffer
    stdin().read_line(&mut string_in).unwrap(); // Read from stdin and fill the buffer
    println!("The input you entered is: {}", string_in.trim()); // Output after trimming

    /*********************************************************
     * INTEGER INPUT
     * -------------------------------------------------------
     * Rust doesn't have dynamic typing, so we must explicitly
     * parse input from `String` to an integer type like `i32`.
     * `parse::<i32>()` will return a Result which we `unwrap()`.
     * Using `.trim()` is essential to remove `\n` or `\r\n`.
     *********************************************************/
    println!("\nPlease enter an integer value:");
    let mut digit_in = String::new();
    stdin().read_line(&mut digit_in).unwrap();
    let x: i32 = digit_in.trim().parse().expect("Input not an integer");
    println!("The integer you entered is: {}", x);

    /*********************************************************
     * CHARACTER INPUT
     * -------------------------------------------------------
     * Characters (`char`) in Rust are 4-byte Unicode scalars.
     * Since there's no `read_char()` function, we read a full
     * line as a `String` and extract the first character.
     *
     * Method 1: Using `.chars().next()`
     * Method 2: Using `.as_bytes()` and converting first byte
     *********************************************************/
    println!("\nPlease enter a single character:");
    let mut char_in = String::new();
    stdin().read_line(&mut char_in).unwrap();

    // Method 1: Safe way to extract first character
    let ch1: char = char_in.trim().chars().next().expect("No character entered");
    println!("The character (method 1) you entered is: {}", ch1);

    // Method 2: Raw byte access (not Unicode-safe for multibyte chars)
    let ch2: char = char_in.trim().as_bytes()[0] as char;
    println!("The character (method 2) you entered is: {}", ch2);

    /*********************************************************
     * FLOATING POINT INPUT
     * -------------------------------------------------------
     * `f32` and `f64` are used for floats. Just like integers,
     * parsing is needed to convert a `String` to float.
     * `f64` offers higher precision and is generally preferred.
     *********************************************************/
    println!("\nPlease enter a floating point number:");
    let mut float_in = String::new();
    stdin().read_line(&mut float_in).unwrap();
    let f: f64 = float_in.trim().parse().expect("Invalid float");
    println!("The float you entered is: {}", f);

    /*********************************************************
     * SAFER INPUT USING MATCH
     * -------------------------------------------------------
     * Here, instead of crashing the program with `unwrap()`,
     * we safely handle errors using `match`.
     * Useful when building robust CLI programs.
     *********************************************************/
    println!("\nEnter another integer (safe input):");
    let mut another_input = String::new();

    // Read the line, and handle the result safely
    match stdin().read_line(&mut another_input) {
        Ok(_) => {
            // Try parsing the input as i32
            match another_input.trim().parse::<i32>() {
                Ok(val) => println!("You entered: {}", val),
                Err(_) => println!("That wasn't a valid integer!"),
            }
        }
        Err(e) => println!("Failed to read input: {}", e),
    }

    /*********************************************************
     * USING print! WITH stdout().flush()
     * -------------------------------------------------------
     * Unlike `println!`, `print!` doesn’t add a newline,
     * and output might not appear unless we flush the buffer.
     * Use `stdout().flush()` to force output to appear.
     *********************************************************/
    print!("\nEnter a string (flush example): ");
    io::stdout().flush().unwrap(); // Flush ensures the print is shown before input
    let mut flushed_input = String::new();
    stdin().read_line(&mut flushed_input).unwrap();
    println!("You typed: {}", flushed_input.trim());

}
