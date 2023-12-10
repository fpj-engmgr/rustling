fn main() {
    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    let average = ((a as f64) + b + (c as f64)) / 3.0;

    println!("average of {}, {} and {} is {}", a, b, c, average);

    assert_eq!(average, 45.1);
    println!("Test passed!");
}
