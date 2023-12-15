use std::io;
use rand::prelude::*;

fn main() {

    let secret_num = thread_rng().gen_range(1..100);
    let mut count: i32 = 1;

    loop {
        let mut guess = String::new();

        println!("Enter a number: ");
        io::stdin().read_line(&mut guess).expect("Failed to read input line.");

        let guess: u32 = guess.trim().parse().expect("Failed to parse the guess.");

        if guess < secret_num {
            println!("Guess #{} of {} is low", count, guess);
            count += 1;
        } else if guess > secret_num {
            println!("Guess #{} of {} is high", count, guess);
            count += 1;
        } else {
            println!("You guess the number was {} in {} attempts!", secret_num, count);
            break;
        }
    } 
}
