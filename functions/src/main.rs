fn main() {
    say_hello();
    let x = 1;
    let y = 26;
    say_the_sum(x,y);

    let p = 238;
    let q = 273;

    let r = return_a_product (p, q);

    println!("{} * {} equals {}", p, q, r);
}

fn say_hello() {
    println!("Hello friendly Rustler!");
}

fn say_the_sum(a: u8, b: u8) {
    println!("The sum is {}", a + b);
}

fn return_a_product(a: i16, b: i16) -> i32 {
    let mut prod: i32 = 0;
    prod = a as i32 * b as i32;
    return prod;
}