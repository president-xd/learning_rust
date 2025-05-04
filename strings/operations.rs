fn main() {


    // Here we will look at different operations performed on strings.

    // 1. Reversing String
    {
        let string1: String = "First".to_string();
        let mut reversed: String = String::new();

        // String's characters in reverse order
        for x in string1.chars().rev(){
            reversed.push(x);
        }

        // Printing the reverse string
        println!("The reversed String of {} is {}",string1 , reversed);
    }

    // 2. Searching string in a long sentence (using .contains() function)
    {
        let string2: String = "Second".to_string();
        let sentence: String = "This is the thing that \
        I was a for second time. And Second time was not that good.".to_string();

        /*
            - using build in function for checking the availability of the string in the sentence
            - we can also use .find() function to find any string in sentence too.
        */

        let found: bool = sentence.contains(&string2); // variable to store the result of found / not found

        // Condition to check
        if found{
            println!("The string found in the sentence..!!");
        } else {
            println!("The string not found in the sentence..!");
        }
    }

    // 3. Searching string in a long sentence (using .find() function)
    {
        let string1: String = "to find".to_string();
        let sentence: String = "I love to find".to_string();

        /*
        - Little explanation regarding is_some() function
            - Returns true if the variant is Some , indicate the /
                presence of a value. is_none() : Returns true if the /
                    variant is None , indicating the absence of a value.
            - Basically, in enum data types there are two things:
                Ex:
                    pub enum Option<T> {
                        None,
                        Some(T),
                    }
              None and Some are the variants of the enum, that is, /
                a value with type Option<T> can either /
                    be a None, or it can be a Some containing a value of type T.
        */
        let found: bool = sentence.find(&string1).is_some();

        if found {
            println!("The string found in the sentence..!");
        } else {
            println!("The string not found in the sentence..!");
        }
    }

    // 4. lowercase to uppercase / uppercase to lowercase
    {
        let string1: String = "FiRst".to_string();
        let string2: String = "second".to_string();
        let string3: String = "FIRST".to_string();

        println!("The lowercase of string3 is {}", string3.to_lowercase());
        println!("The uppercase of string2 is {}", string2.to_uppercase());
        //println!("The  case of string1 is {}", string1.to_lowercase());
    }
    
        // 5. Joining of strings
    {
        let string1: String = "first".to_string();
        let string2: String = "second".to_string();
        let concatenate: String = string1 + " " + &string2;

        println!("The concatenate of string1 & string2 is {}", concatenate);
    }

    // 6. Another way of concatenation of strings
    {
        let string1: String = "first".to_string();
        let string2: String = "second".to_string();
        let concatenate: String = format!("{} {}", string1, string2);

        println!("The concatenate of string1 & string2 is {}", concatenate);
    }

    // Example: Declaration of Vectors in Rust
    {
        let collected_iterator: Vec<i32> = (0..10).collect();
        println!("Collected (0..10) into: {:?}", collected_iterator);

        let vec: Vec<String> = Vec::new();
        println!("Strings collected into: {:?}", vec);
    }

    // 7. Splitting Sentences into words using vectors
    {
        let sentence: String = "This is first our form of things".to_string();
        let splited_words: Vec<&str> = sentence.split(" ").collect();

        println!("The splited words of strings collected into: {:?}", splited_words);
    }
}
