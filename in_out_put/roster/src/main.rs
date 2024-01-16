
use std::env;
use std::fs;

fn main() {
    let numargs = env::args().len();

    if numargs < 3 {
        println!("This program requires 2 arguments\n\tfilename\n\tcandidate name");
        return;
    }
//    println!("{} arguments were supplied", numargs);
//    println!("Zeroeth argument...{}", env::args().nth(0).unwrap());

    let fname = env::args().nth(1).unwrap();
    let cname = env::args().nth(2).unwrap();

    let walkers =  fs::read_to_string(fname).unwrap();
    for lunar in walkers.lines() {
        if lunar == cname {
            println!("{} has walked on the Moon!", cname);
            return;
        }
    }
    println!("{} hasn't walked on the moon yet", cname);
}
