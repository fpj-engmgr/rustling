use std::io;

use rand;

fn main() {
    let mut buffer = String::new();
    println!("Enter a message:");
    io::stdin().read_line(&mut buffer);
    println!("buffer is {}", buffer);

    let number: i32 = buffer.trim().parse().unwrap();
    println!("number + 1 is {}", number + 1);

    let rando = rand::random::<f64>();
    println!("rando [0-100] is {:7.5}", rando * 100.0);
}