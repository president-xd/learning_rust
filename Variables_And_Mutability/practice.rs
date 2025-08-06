fn main() {
    // ============================================================
    //                  Welcome / Chapter Introduction
    // ============================================================

    println!("################### Chapter 3 ###################");
    println!("-------------------------------------------------");
    println!("Hello, Rustaceans!");
    print!("We will learn Variables, and Mutability in Rust.");
    println!(" Let's get started!");
    println!("-------------------------------------------------");
    println!("Website Link: https://www.president-xd.com/blog/common_programming_concepts#variables-and-mutability-in-rust-");
    println!("-------------------------------------------------\n");

    // ============================================================
    // 1. Variable Binding (Immutable by default in Rust)
    // ============================================================
    {
        println!("1. Variable Binding: Basic");
        println!(" ");

        // In Rust, variables are immutable by default.
        // Here, we're declaring a variable `x` with the value 5.
        // Since it's not mutable, its value cannot be changed after this.
        let x = 5;
        println!("The value of x is: {}", x);

        // Trying to do something like `x = 10;` would cause a compile-time error.
    }
    println!(" ");

    // ============================================================
    // 2. Mutable Variables
    // ============================================================
    {
        println!("2. Mutable Variables");
        println!(" ");

        // By using `mut`, we allow the value of the variable to change.
        // Here we initialize `counter` with 0, then increment it.
        let mut counter = 0;
        println!("The initial value of counter is: {}", counter);

        counter += 1; // Equivalent to: counter = counter + 1;
        println!("The value of counter after incrementing is: {}", counter);
    }
    println!(" ");

    // ============================================================
    // 3. Shadowing Practice
    // ============================================================
    {
        println!("3. Shadowing Practice");
        println!(" ");

        // Shadowing allows you to declare a new variable with the same name.
        // The new variable can have a different value or even a different type.
        let variable = 10;
        println!("The initial value of variable is: {}", variable);

        // This `let` shadows the previous `variable` and assigns it a new value.
        let variable = 55 + variable;
        println!("The value of variable after shadowing is: {}", variable);

        // Unlike `mut`, shadowing creates a new binding. This helps with immutability and safe transformations.
    }
    println!(" ");

    // ============================================================
    // 4. Constants
    // ============================================================
    {
        println!("4. Constants");
        println!(" ");

        // Constants are always immutable, must be typed, and must be declared in global or function scope.
        // Useful for values that should never change throughout the program.
        const MAX_POINTS: i32 = 100000;
        println!("The value of MAX_POINTS is: {}", MAX_POINTS);

        // Note: You cannot use `mut` with constants. They are not allowed to change.
    }
    println!(" ");

    // ============================================================
    // 5. Scope Check
    // ============================================================
    {
        println!("5. Scope Check");
        println!(" ");

        // `variable_out` is declared in the outer scope of this block.
        let mut variable_out = 20;

        {
            // `variable_in` exists only within this inner block.
            let variable_in = 30;

            // Inside the scope, we can access both variables.
            println!("The value of variable_in inside the scope is: {}", variable_in);
            variable_out += variable_in;
            println!("The value of variable_out after adding variable_in is: {}", variable_out);
        }

        // After the inner block ends, `variable_in` is no longer accessible.
        println!("The value of variable_out outside the scope is: {}", variable_out);

        // Uncommenting the following line would cause a compile-time error:
        // println!("The value of variable_in outside the scope is: {}", variable_in);
    }
    println!(" ");

    // ============================================================
    // 6. Temperature Converter (Celsius to Fahrenheit)
    // ============================================================
    {
        println!("6. Temperature Converter");
        println!(" ");

        // This loop will repeatedly ask the user to input temperature until they enter a valid number.
        loop {
            let mut celsius: String = String::new(); // Create a mutable empty string to store input

            use std::io; // Import standard input/output library
            println!("Please enter the temperature in Celsius:");

            // Read input from the user and store it in `celsius`
            io::stdin()
                .read_line(&mut celsius)
                .expect("Failed to read user input");

            // Convert string input to floating point number
            let celsius: f32 = match celsius.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    // If parsing fails, prompt user again
                    println!("Kindly enter valid temperature..");
                    continue;
                }
            };

            // Apply temperature conversion formula
            let fahrenheit: f32 = ((celsius * 9.0) / 5.0) + 32.0;

            println!("The temperature in Fahrenheit is {}", fahrenheit);
            break; // Exit the loop after successful conversion
        }
    }
    println!(" ");

    // ============================================================
    // 7. Simple Counter Loop
    // ============================================================
    {
        println!("7. Simple Counter Loop");
        println!(" ");

        // A simple while loop to count from 0 to 10
        let mut counter: i32 = 0;

        while counter <= 10 {
            println!("Counting: {}", counter);
            counter += 1;
        }

        // Demonstrates how to work with mutable state in a loop
    }
    println!(" ");

    // ============================================================
    // 8. Shadowing With Types
    // ============================================================
    {
        println!("8. Shadowing With Types");
        println!(" ");

        // Start with an integer value
        let secret_number: i32 = 123;

        // Shadow `secret_number` with a float version (after type casting)
        let secret_number: f32 = secret_number as f32;

        println!("The secret number is {}", secret_number);

        // This is an example of how shadowing can change not just values, but also variable types
    }
    println!(" ");

    // ============================================================
    // 9. Guessing Output (Shadowing and Scope)
    // ============================================================
    {
        println!("9. Guessing Output");
        println!(" ");

        println!("Code : ");

        // Print out the code snippet for which we will predict the output
        print!(
            "fn main() {{
                let x = 5;
                let x = x + 1;
                {{
                    let x = x * 2;
                    println!(\"Inner 'x': {{}}\", x);
                }}
                println!(\"Outer 'x': {{}}\", x);
            }}\n"
        );
        println!(" ");

        println!("Output : ");

        // Explain expected behavior due to inner and outer scope shadowing
        println!(
            "
            Inner x: 12
            Outer x: 6;
            "
        )

        // Explanation:
        // - Outer `x` starts at 5, then is shadowed with `x + 1 = 6`
        // - Inside the block, another `x` shadows it and becomes `6 * 2 = 12`
        // - Outer print still accesses x = 6
    }
}
