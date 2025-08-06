// Task 4: Function
fn c_2_f_temperature(celsius: f64) -> f64 {
    // Variable definition and formula calculation
    let fahrenheit: f64 = (celsius * (9_f64 / 5_f64)) + 32_f64;
    return fahrenheit;
}

// Task 5: Function
fn swap_tuple(tuple: (i32, bool)) -> (bool, i32) {
    let tuple: (bool, i32) = (tuple.1, tuple.0);
    return tuple;
}

// Task 6: Function
fn calc_average(array: [i32; 5]) -> f64 {
    // Creating a variable to save the average
    let mut average: f64 = 0_f64;

    // Loop for continuous calulation average
    for i in array {
        average += i as f64;
    }

    // Size of the Array
    let size: usize = array.len();

    // Calculating Average
    let average: f64 = average / size as f64;

    // This format is used to return the value:
    // Like 'return average;'
    average
}

// Entry Point Function
fn main() {
    // Task 1: Basic Scaler Operations
    // Description:
    /*
        Write a program that:
            - Declares an i32, u8, f64, char, and bool.
            - Prints them using println!.
            - Performs one arithmetic operation per numeric type and prints the result.
    */

    // Solution:
    {
        let integer: i32 = 56;
        let unsigned: u8 = 5;
        let floating_point: f64 = 8.99;
        let character: char = '9';
        let condition: bool = true;

        // Printig them
        println!("The value of integer variable is : {integer}");
        println!("The value of unsigned variable is : {unsigned}");
        println!("The value of floating_point variable is : {floating_point}");
        println!("The value of character variable is : {character}");
        println!("The value of condition variable is : {condition}");

        // Arthimatic Operation
        let equation: f64 = ((integer as f64 + unsigned as f64) * floating_point) / 45_f64;

        // Printing the value of the equation
        println!(
            "The Answer of {{integer + unsigned * floating_point / 45}} is {}",
            equation
        );
    }

    // Task 2
    // Description:
    /*
        Tuple Access
            - Create a tuple of different types: an integer, float, boolean, and char.
            - Access and print each element using indexing.
            - Destructure the tuple and print all values using variable bindings.
    */

    // Solution:
    {
        // Tuple Created
        let tuple: (i32, f64, bool, char) = (34, 566.00, true, 'A');

        // Accessing and Printing the value
        println!("The first value of tuple is : {}", tuple.0);
        println!("The second value of tuple is : {}", tuple.1);
        println!("The third value of tuple is : {}", tuple.2);
        println!("The fourth value of tuple is : {}", tuple.3);

        // Destructuring the tuple
        let (first, second, third, fourth) = tuple;

        // Printing the tuple
        println!("The value of first varaible is : {}", first);
        println!("The value of second varaible is : {}", second);
        println!("The value of third varaible is : {}", third);
        println!("The value of fourth varaible is : {}", fourth);
    }

    // Task 3: Array Slicing
    // Description:
    /*
        - Create an array of 5 integers.
        - Print the first element.
        - Create and print a slice of the last three elements.
    */

    // Solution
    {
        // Creating the Array of 5 Integers
        let consider: [i32; 5] = [0, 1, 2, 3, 4];

        // Printing the first element
        println!("The first element of the Array is {}", consider[1]);

        // Creating and printing the slice of last three elements
        for index in 2..4 {
            println!(
                "The element on index {index} of Array is {}",
                consider[index]
            );
        }
    }

    // Task 4: Celsius to Fahrenheit
    // Description:
    /*
    Create a function that takes a f64 (Celsius) and returns Fahrenheit using the formula.
        - Use type annotations and float arithmetic.
        - Include user input using std::io.
    */

    // Solution:
    {
        // Loop for Valid Input
        loop {
            let mut celsius: String = String::new();

            //use std::io;

            // Prompting String for User input
            println!("Enter the Celsius Temperature to convert : ");

            // Taking input from the user
            std::io::stdin()
                .read_line(&mut celsius)
                .expect("Failed to read the line..\n");

            // Parsing the input for valid number
            let celsius: f64 = match celsius.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Kindly enter valid number");
                    continue;
                }
            };

            // Variable definition and formula calculation
            let fahrenheit: f64 = (celsius * (9_f64 / 5_f64)) + 32_f64;

            // Printing the output and exiting the loop
            println!("The Fahrenheit Temperature is : {}", fahrenheit);
            break;
        }

        // Or the Functional Approach is here
        let mut celsius: String = String::new();

        //use std::io;

        // Prompting String for User input
        println!("Enter the Celsius Temperature to convert : ");

        // Taking input from the user
        std::io::stdin()
            .read_line(&mut celsius)
            .expect("Failed to read the line..\n");

        // Parsing the input for valid number
        let celsius: f64 = celsius.trim().parse().expect("Enter valid number");

        let farenheit: f64 = c_2_f_temperature(celsius);

        // Printing the temperature in fahrenheit
        println!("The temperature in Fahrenheit is : {}", farenheit);
    }

    // Task 5: Tuple Swap
    // Description:
    /*
        Write a function that:
        - Takes a tuple (i32, bool) as input.
        - Returns a tuple (bool, i32) by swapping elements.
    */

    // Solution:
    {
        // Non Functional Way
        {
            // Tuple Created
            let a_tuple: (i32, bool) = (32, true);

            // Before Swap
            println!("Before Swap: ");
            println!("The tuple first element : {}", a_tuple.0);
            println!("The tuple second element : {}", a_tuple.1);

            // Shadowing to gain swap
            let a_tuple: (bool, i32) = (a_tuple.1, a_tuple.0);

            // After Swap
            println!("After Swap: ");
            println!("The tuple first element : {}", a_tuple.0);
            println!("The tuple second element : {}", a_tuple.1);
        }

        // Function Approach

        // Tuple Created
        let tuple: (i32, bool) = (32, true);

        // Before Swap
        println!("Before Swap: ");
        println!("The tuple first element : {}", tuple.0);
        println!("The tuple second element : {}", tuple.1);

        // Function for Swaping
        let tuple: (bool, i32) = swap_tuple(tuple);

        // After Swap
        println!("After Swap: ");
        println!("The tuple first element : {}", tuple.0);
        println!("The tuple second element : {}", tuple.1);
    }

    // Task 6: Average of Array
    // Description:
    /*
        Write a function that:
            - Takes an array of f64 with 5 elements.
            - Returns the average as f64.
    */

    // Solution:
    {
        // Non-Functional Approach
        {
            // Array Creation
            let array: [i32; 5] = [3, 55, 66, 77, 55];

            // Creating a variable to save the average
            let mut average: f64 = 0_f64;

            // Loop for continuous calulation average
            for i in array {
                average += i as f64;
            }

            // Size of the Array
            let size: usize = array.len();

            // Calculating Average
            let average: f64 = average / size as f64;

            // Printing the output
            println!("The Average of the Array is : {}", average);
        }

        // Function Approach

        // Array Creation
        let array: [i32; 5] = [3, 55, 66, 77, 55];

        // Function to calulate the average
        let average: f64 = calc_average(array);

        // Printing the output
        println!("The Average of the Array is : {}", average);
    }
}
