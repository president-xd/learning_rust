// ============================================================
//                Chapter 1: Hello World & Cargo
// ============================================================

fn main() {
    println!("################### Chapter 1 ###################");
    println!("-------------------------------------------------");
    println!("Hello, Rustaceans!");
    println!("We will explore how to write your first program in Rust,");
    println!("and how to manage it using Cargo, Rust's package manager.");
    println!("-------------------------------------------------\n");

    // ========================================================
    // Task 1: Your First Rust Program
    // ========================================================
    {
        println!("1. First Rust Program:");
        println!(" ");

        // This is the most basic executable Rust program.
        // Every Rust program starts with the `main` function.
        // `println!` is a macro used for printing output to the console.
        println!("Hello, Rustacean!");
    }
    println!(" ");

    // ========================================================
    // Task 2: Syntax Error Practice
    // ========================================================
    {
        println!("2. Syntax Error (Uncomment to test):");
        println!(" ");

        // Rust expects every statement to end with a semicolon (;)
        // Uncomment the following to see a compile-time error
        //
        // println!("This line will cause an error if no semicolon")
    }
    println!(" ");

    // ========================================================
    // Task 3: Multiple println! Statements
    // ========================================================
    {
        println!("3. Multiple println! Usage:");
        println!(" ");

        // You can use multiple `println!` macros in one program.
        // This helps understand code execution order and formatting.
        println!("Rust is fast.");
        println!("Rust is memory-safe.");
        println!("Rust is fun!");
    }
    println!(" ");

    // ========================================================
    // Task 4: Simulating Cargo Project Structure
    // ========================================================
    {
        println!("4. Simulating Cargo Project:");
        println!(" ");

        // In a real project, this code would live inside:
        // └── src/main.rs
        //
        // Cargo is used to initialize, build, and run Rust projects.
        // To create a new Cargo project, you use:
        //     cargo new hello_project
        //
        // This command creates the following structure:
        //
        // hello_project/
        // ├── Cargo.toml        --> Project metadata & dependencies
        // └── src/
        //     └── main.rs       --> Main source code
        //
        println!("Cargo project structure simulated. See comments for layout.");
    }
    println!(" ");

    // ========================================================
    // Task 5: Modify and Print from Cargo Project
    // ========================================================
    {
        println!("5. Output From Cargo Project:");
        println!(" ");

        // In a Cargo project, you can modify `src/main.rs` like this:
        println!("Hello from Cargo project!");
    }
    println!(" ");

    // ========================================================
    // Task 6: Understanding Cargo.toml
    // ========================================================
    {
        println!("6. Cargo.toml Example (See comments):");
        println!(" ");

        // The Cargo.toml file contains project configuration:
        //
        // [package]
        // name = "hello_project"
        // version = "0.1.0"
        // edition = "2021"
        //
        // [dependencies]
        //
        // Dependencies from crates.io are added here.
        // Example: rand = "0.9.2"
        println!("Cargo.toml defines project metadata and dependencies.");
    }
    println!(" ");

    // ========================================================
    // Task 7: Building with Cargo
    // ========================================================
    {
        println!("7. How to Build and Run Using Cargo:");
        println!(" ");

        // From your terminal inside a Cargo project:
        //
        // cargo build       --> Compiles the code
        // cargo run         --> Compiles and runs the code
        // cargo build --release --> Compiles optimized release binary
        //
        println!("Use `cargo run` to build and execute your code.");
    }
    println!(" ");

    // ========================================================
    // Final Summary
    // ========================================================
    {
        println!("Summary:");
        println!(" ");

        println!("- `fn main()` is the program entry point.");
        println!("- `println!` prints messages to the console.");
        println!("- Cargo helps manage building, dependencies, and structure.");
        println!("- `Cargo.toml` defines your project's metadata and crates.");
        println!("- Use `cargo new`, `cargo build`, and `cargo run` to work efficiently.");
    }
}
