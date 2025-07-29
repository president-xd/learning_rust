fn main() {
    println!("################### Chapter 3 ###################");
    println!("-------------------------------------------------");
    println!("Hello, Rustaceans!");
    print!("We will learn Variables, and Mutability in Rust.");
    println!(" Let's get started!");
    println!("-------------------------------------------------");
    println!("Website Link: https://www.president-xd.com/blog/common_programming_concepts");
    println!("-------------------------------------------------\n");

    // Tasks:

    // 1. Variable Binding: Basic
    {
        println!("1. Variable Binding: Basic");
        println!(" ");

        let x = 5;
        println!("The value of x is: {}", x);
    }
    println!(" ");

    // 2. Mutable Variable
    {
        println!("2. Mutable Variables");
        println!(" ");

        let mut counter = 0;
        println!("The initial value of counter is: {}", counter);
        counter += 1;
        println!("The value of counter after incrementing is: {}", counter);
    }

    println!(" ");

    // 3. Shadowing Practice
    {
        println!("3. Shadowing Practice");
        println!(" ");

        let variable = 10;
        println!("The initial value of variable is: {}", variable);
        let variable = 55 + variable;
        println!("The value of variable after shadowing is: {}", variable);
    }

    println!(" ");

    // 4. Constants
    {
        println!("4. Temperature Converter");
        println!(" ");

        const MAX_POINTS: i32 = 100000;
        println!("The value of MAX_POINTS is: {}", MAX_POINTS);
    }

    println!(" ");

    // 5. Scope Check
    {
        println!("5. Scope Check");
        println!(" ");

        let mut variable_out = 20;
        {
            let variable_in = 30;
            println!(
                "The value of variable_in inside the scope is: {}",
                variable_in
            );
            variable_out += variable_in;
            println!(
                "The value of variable_out after adding variable_in is: {}",
                variable_out
            );
        }

        println!(
            "The value of variable_out outside the scope is: {}",
            variable_out
        );
        // If I print the variable_in here, it will cause an error because it is out of scope.
        // println!("The value of variable_in outside the scope is: {}", variable_in); //
    }

    println!(" ");

    // 6. Temperature Converter
    {
        println!("6. Temperature Converter");
        println!(" ");
        loop {
            let mut celsius: String = String::new();

            // using standard library to read user input.
            use std::io;
            println!("Please enter the temperature in Celsius:");
            io::stdin()
                .read_line(&mut celsius)
                .expect("Failed to read user input");

            let celsius: f32 = match celsius.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Kindly enter valid temperature..");
                    continue;
                }
            };

            let fahrenheit: f32 = ((celsius * 9.0) / 5.0) + 32.0;

            println!("The temperature in fahrenheit is {}", fahrenheit);
            break;
        }
    }

    println!(" ");

    // 7. Simple Counter Loop
    {
        println!("7. Simple Counter Loop");
        println!(" ");
        let mut counter: i32 = 0;

        while counter <= 10 {
            println!("Counting: {}", counter);
            counter += 1;
        }
    }

    println!(" ");

    // 8. Shadowing With Types
    {
        println!("8. Shadowing With Types");
        println!(" ");

        let secret_number: i32 = 123;
        let secret_number: f32 = secret_number as f32;
        println!("The secret number is {}", secret_number);
    }

    println!(" ");

    // 9. Guessing Output
    {
        println!("9. Guesing Output : ");
        println!(" ");

        println!("Code : ");

        // Code Snippet
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

        // Code Snippet Output
        println!(
            "
            Inner x: 12
            Outer x: 6;
            "
        )
    }
}
