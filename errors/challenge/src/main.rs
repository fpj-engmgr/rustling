use std::io;
use rand::prelude::*;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..100);
    println!("the number is {}", secret_number);

    println!("I'm thinking of a number between 1 and 100...");
    println!("Guess the number:");
    loop {
        let mut buffer = String::new();
        let guess = match io::stdin().read_line(&mut buffer) {
            Ok(_) => match buffer.trim().parse::<u32>() {
                    Ok(value)   => value, // it worked
                    Err(_)      => {
                        println!("\nFailed to parse your input. Please try again: ");
                        continue
                    }
            
                }
            Err(_) => {
                println!("\nFailed to read your input. Please guess again: ");
                continue
            }

        };

        if guess > secret_number {
            println!("\n{} is too high! Guess lower:", guess);
        } else if guess < secret_number {
            println!("\n{} is too low! Guess higher:", guess);
        } else {
            println!("\nYou got it! The secret number was {}.", secret_number);
            break;
        }
    }    
}
