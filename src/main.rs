use colored::*;
use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");

    // mut - defines a variable to be mutable
    // String::new() - creates a new string
    let secret_number = rand::thread_rng().gen_range(1..100);

    loop {
        let mut guess = String::new();
        println!("Please input your guess.");
        io::stdin()
            .read_line(&mut guess)
            .expect("Error while read line");
        println!("You guessed {guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
            Ordering::Greater => {
                println!("{}", "Guess smaller".yellow());
            }
            Ordering::Less => {
                println!("{}", "Guess larger".yellow());
            }
        }
    }
}
