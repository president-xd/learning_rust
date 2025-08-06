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

// Task 7: Function
fn array_reversal(array: [i32; 5]) -> [i32; 5] {
    // Created new array to store elements
    let mut array1: [i32; 5] = [0; 5];

    // Length of the Array
    let size = array.len();

    // Reversing the Array
    for i in 0..array.len() {
        array1[size - i - 1] = array[i];
    }

    return array1;
}

// Fibonacci Series returning tuple of nth and n+1th term
fn fib_calc(n: u32) -> (usize, usize) {
    let mut a: usize = 0;
    let mut b: usize = 1;

    if n == 0 {
        return (a, b);
    }

    for _ in 1..n {
        let c = a + b;
        a = b;
        b = c;
    }

    (a, b)
}

// Task 10: Function
fn matrix_row_sum(matrix: [[i32; 3]; 3]) -> [i32; 3] {
    let mut sum_array: [i32; 3] = [0; 3];

    for i in 0..3 {
        for j in 0..3 {
            if i == 0 {
                sum_array[0] += matrix[0][j];
            } else if i == 1 {
                sum_array[1] += matrix[1][j];
            } else if i == 2 {
                sum_array[2] += matrix[2][j];
            } else {
                continue;
            }
        }
    }

    sum_array
}

// Task 11: Function
fn rgb_to_hex(rgb: (u8, u8, u8)) -> String {
    let hex_rgb: (String, String, String) = (
        format!("{:#X}", rgb.0),
        format!("{:X}", rgb.1),
        format!("{:X}", rgb.2),
    );
    let red_hex: String = hex_rgb.0;
    let green_hex: String = hex_rgb.1;
    let blue_hex: String = hex_rgb.2;

    let hex_rgb: String = red_hex + &green_hex + &blue_hex;

    hex_rgb
}

// Entry Point Function
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
    println!("Website Link: https://www.president-xd.com/blog/common_programming_concepts");
    println!("-------------------------------------------------\n");

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
        println!("The first element of the Array is {}", consider[0]);

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

    // Task 7: Array Reversal
    // Description:
    /*
        Create a function that:
            - Takes a array of integers.
            - Reverses it in-place (no Vec, no .reverse() shortcut).
            - Prints before and after reversal.
    */
    // Solution:
    {
        // Non-funtional Approach
        {
            // Array Created
            let array: [i32; 5] = [32, 45, 66, 34, 22];

            // Printing the Array before reversal
            println!("Array before traversal");
            for i in 0..5 {
                println!("Array index {i} element : {}", array[i]);
            }

            // Created new array to store elements
            let mut array1: [i32; 5] = [0; 5];

            // Length of the Array
            let size = array.len();

            // Reversing the Array
            for i in 0..array.len() {
                array1[size - i - 1] = array[i];
            }

            // Printing the Array after reversal
            println!("Array after reversal: ");
            for i in 0..5 {
                println!("Array index {i} element : {}", array1[i]);
            }
        }

        // Functional Approach
        // Array Created
        let array: [i32; 5] = [32, 45, 66, 34, 22];

        // Printing the Array before reversal
        println!("Array before traversal");
        for i in 0..5 {
            println!("Array index {i} element : {}", array[i]);
        }

        // Shadowed Variable for storing the reversed array
        let array: [i32; 5] = array_reversal(array);

        // Printing the Array after reversal
        println!("Array after reversal: ");
        for i in 0..5 {
            println!("Array index {i} element : {}", array[i]);
        }
    }

    // Task 8: Fibonacci with Tuple Return
    // Description:
    /*
        Write a function:
            - Takes a number n: u32
            - Returns a tuple containing the nth and n + 1th the fibonacci numbers.
    */
    // Solution:
    {
        // Non Functional Approach
        {
            let mut a: usize = 0;
            let mut b: usize = 1;

            println!("Enter the number to find the fibonacci series: ");
            let mut n: String = String::new();
            std::io::stdin()
                .read_line(&mut n)
                .expect("Failed to read the line..\n");
            let n: u32 = n.trim().parse().expect("Please enter a valid number");

            if n == 0 {
                println!("The fibonacci number at index {} is {}", n, a);
                println!("The fibonacci number at index {} is {}", n + 1, b);
            }

            for _ in 1..n {
                let c = a + b;
                a = b;
                b = c;
            }

            (a, b);

            let fib_tuple: (usize, usize) = (a, b);
            println!("The fibonacci number at index {} is {}", n, fib_tuple.0);
            println!("The fibonacci number at index {} is {}", n + 1, fib_tuple.1);
        }

        // Functional Approach
        println!("Enter the nth term to find fibonacci series: ");
        let mut n: String = String::new();
        std::io::stdin()
            .read_line(&mut n)
            .expect("Failed to read the line");

        let n: u32 = n.trim().parse().expect("Kindly enter valid input");

        let return_tuple: (usize, usize) = fib_calc(n);

        println!("The {}th term of Fibonacci Series : {}", n, return_tuple.0);
        println!(
            "The {}th term of Fibonacci Series : {}",
            n + 1,
            return_tuple.1
        );
    }

    // Task 9: Type Conversion & Overflows
    // Description:
    /*
        Write a program that:
            - Reads a number from input as String, parses it to u8, then converts to f64.
            - Show what happens when the user enters a number > 255 (handle with Result).
    */
    // Solution:
    {
        loop {
            let mut input: String = String::new();

            println!("Enter the number to read : ");

            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read the input.");

            let mut input: u8 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Overflow, Kindly enter valid number.");
                    continue;
                }
            };

            let input: f64 = input as f64;
            println!("The input you entered was : {}", input);
            break;
        }
    }

    // Task 10: Matrix Row Sum
    // Description:
    /*
        - Create a 3x3 matrix using a nested array: [[i32; 3]; 3]
        - Write a function that returns a Vec<i32> containing the sum of each row.
    */
    // Solution:
    {
        // Non-Functional Approach
        {
            let matrix: [[i32; 3]; 3] = [[5; 3]; 3];

            let mut row_sum: [i64; 3] = [0; 3];

            for i in 0..3 {
                for j in 0..3 {
                    if i == 0 {
                        row_sum[0] = row_sum[0] + matrix[0][j] as i64;
                    } else if i == 1 {
                        row_sum[1] = row_sum[1] + matrix[1][j] as i64;
                    } else if i == 2 {
                        row_sum[2] = row_sum[2] + matrix[2][j] as i64;
                    }
                }
            }

            println!("The sum of the first row is : {}", row_sum[0]);
            println!("The sum of the second row is : {}", row_sum[1]);
            println!("The sum of the third row is : {}", row_sum[2]);
        }

        // Functional Approach
        let matrix: [[i32; 3]; 3] = [[5; 3]; 3];

        let sum_array: [i32; 3] = matrix_row_sum(matrix);

        println!("The sum of the first row is : {}", sum_array[0]);
        println!("The sum of the second row is : {}", sum_array[1]);
        println!("The sum of the third row is : {}", sum_array[2]);
    }

    // Task 11: RGB Color Parser
    // Description:
    /*
        - Given a tuple of (u8, u8, u8), representing RGB.
        - Write a function that converts it to a hex string like "#FF00FF".
    */

    // Solution:
    {
        // Non Functional Approach
        {
            let rgb: (u8, u8, u8) = (0, 0, 255);

            let hex_rgb: (String, String, String) = (
                format!("{:#X}", rgb.0),
                format!("{:X}", rgb.1),
                format!("{:X}", rgb.2),
            );
            let red_hex: String = hex_rgb.0;
            let green_hex: String = hex_rgb.1;
            let blue_hex: String = hex_rgb.2;

            let hex_rgb: String = red_hex + &green_hex + &blue_hex;
            println!("The HEX value of the RGB color is : {}", hex_rgb);
        }

        // Functional Approach

        // Tuple creation
        let rgb: (u8, u8, u8) = (23, 255, 255);

        // Function to convert them in HEX value and return string
        let rgb_hex: String = rgb_to_hex(rgb);

        println!("The HEX value of the RGB Colors is : {}", rgb_hex);
    }

}
