fn main() {

    // Different Ways of declaring strings

    // 1. using &str to make it string
    /*
    - &str = "string1";
        - This is a string slice.
        - Think of it like a pointer to constant character array in C++:
          Example:
            const char* string1 = "string1";
        - It doesn't own the string. It just points to a fixed memory location
          (like a literal in program memory).
        - It's fast and lightweight, but cannot be modified.

        - At last: the case is of ownership of this variable.
        - It can be changed using to_owned() function
    */
    let string: &str = "string1";
    // 2. Empty String Declaration
    let string1: &str = "";

    // 3. using String
    /*
    - String = "string2".to_string();
        - This is a heap-allocated string object
        - Think of it like std::string in C++:
          Example:
            std::string string2 = "string2";
        - It owns its memory and can grow (is mutable) or changed (because of ownership).
        - We can modify, append, clone it.

        - At Last: the case is of ownership of this variable.
        - It is already changed, we don't need to_owned() function (for its ownership).
    */
    let string2: String = "string2".to_string();
    // 4. Empty Variable Declaration
    let string3: String = String::new();

    // 5. Using String::from()
    /*
    - Equivalent to: std::string string2 = "string2";
    - Owned, mutable, heap-allocated
    - Another common way of creating a String
    */
    let string4: String = String::from("string2");

    // 6. From other variables (&str to String using to_owned)
    /*
    - Converts a &str to an owned String (alternative to to_string)
    */
    let s_slice: &str = "temporary";
    let string5: String = s_slice.to_owned();

    // 7. Using format! macro (like C++ std::stringstream or sprintf)
    /*
    - Useful for dynamic or formatted strings
    */
    let name = "Rust";
    let string6 = format!("Hello, {}", name); // => "Hello, Rust"

    // 8. From bytes using from_utf8
    /*
    - When working with raw byte buffers (Vec<u8>)
    - Similar to casting byte arrays to strings in C++
    */
    let bytes = vec![82, 117, 115, 116]; // 'R', 'u', 's', 't'
    let string7 = String::from_utf8(bytes).unwrap(); // "Rust"

    // 9. Using repeat() to create repeated patterns
    /*
    - Like creating a string with multiple repeated characters
    */
    let string8 = "abc".repeat(3); // "abcabcabc"
    

    // Printing Strings
    println!("The value of string : {}", string);
    println!("The value of string1 : {}", string1);
    println!("The value of string2 : {}", string2);
    println!("The value of string3 : {}", string3);
    println!("The value of string4 : {}", string4);
    println!("The value of string5 : {}", string5);
    println!("The value of string6 : {}", string6);
    println!("The value of string7 : {}", string7);
    println!("The value of string8 : {}", string8);
    
}
