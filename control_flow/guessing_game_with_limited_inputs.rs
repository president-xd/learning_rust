use std::cmp::Ordering;
use std::io;
use rand::prelude::*;

fn main() {

    let random: u64 = rand::random_range(0..=100);

    for _ in 1..=5 {

        let mut input: String = String::new();

        println!("Guess Number from 1 to 100 : ");

        io::stdin().read_line(&mut input).expect("Failed to read the line..");

        let input: u64 = match input.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, kindly enter valid number.");
                continue;
            }
        };

        match input.cmp(&random){
            Ordering::Less => {
                println!("Too small, Try bigger number..");
                continue;
            } Ordering::Greater => {
                println!("Too big, Try smaller number..");
                continue;
            } Ordering::Equal => {
                println!("You won the game bro, GG :)");
                break;
            }
        }
    }

    println!("The Correct Number was : {random}")
}