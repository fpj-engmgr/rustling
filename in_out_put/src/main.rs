use std::env;

fn main() {
   let numargs = env::args().len();
   if numargs < 3 {
    println!("Error: insufficient number of arguments {} Min = 3", numargs);
        return;
   }
   for (index, argument) in env::args().enumerate() {
        println!("Argument {} is {}", index, argument);
    }

    let arg2 = env::args().nth(2).unwrap();
    println!("2nd argument is {}", arg2);
}
