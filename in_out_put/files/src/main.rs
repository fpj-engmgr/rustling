use std::fs;

fn main() {
    let mut count = 1;
    let contents = fs::read_to_string("planets.txt").unwrap();
//   println!("contents is {}", contents);

    for line in contents.lines() {
        println!("Planet #{} is {}", count, line);
        count += 1;
    }
/*     
    let contents = fs::read("planets.txt").unwrap();
    println!("contents is {:?}", contents);
*/
    let mut speech = String::new();

    speech.push_str("We choose to go to the Moon in this decade\n");
    speech.push_str("and do the other things,\n");
    speech.push_str("not because they are easy,\n");
    speech.push_str("but because they are hard.\n");

    fs::write("speech.txt", speech).expect("Couldn't write to file");

}
