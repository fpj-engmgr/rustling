use std::fs;
use std::env;

fn main() {
    let numargs = env::args().len();

    if numargs < 2 {
        println!("\nPlease provide an input file to parse.\n");
        return;
    }
    let infile = env::args().nth(1).unwrap();

    let mut line_count = 0;
    let parses = match fs::read_to_string(infile).unwrap() {
        Ok(_)   => parses,
        Err(_)      =>  { 
            println!("Can't open this file. Please check it exists\n");
            return
        }
    };

    for parseline in parses.lines() {
        line_count += 1;
        println!("line {} : {}", line_count, parseline);
    }
}
