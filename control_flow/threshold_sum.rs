fn main() {

    let mut threshold: u64 = loop {
        let mut threshold: String = String::new();


        println!("Enter the threshold value : ");
        std::io::stdin().read_line(&mut threshold).expect("Failed to read the line");

        println!("The threshold value was : {threshold}");

        match threshold.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Invalid Input !!");
                continue;
            }
        }
    };

    let mut series_inputs: String = String::new();

    println!("Enter the values (with space difference): ");
    std::io::stdin().read_line(&mut series_inputs).expect("Failed to read the line.");

    println!("Printing the inputs: {}", series_inputs);

    let mut sum: u64 = 0;
    let mut count: u16 = 0;

    for values in series_inputs.trim().split_whitespace() {
        let num: u64 = match values.trim().parse(){
            Ok(number) => number,
            Err(_) => {
                println!("Failed to parse the number, kindly enter valid Unsigned Integer.");
                break;
            }
        };

        count += 1;
        sum += num;

        if sum >= threshold {
            break;
        }

    }

    println!("{} {}", count, sum);
}