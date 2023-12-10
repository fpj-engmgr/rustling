fn main() {
    let mut abit = 0b1111_0101u8;
    println!("Value of abit is {:08b}", abit);
    abit = abit >> 2;
    println!("Value of abit >> 2 is {:08b}", abit);
    abit = abit << 2;
    println!("Value of abit << 2 is {:08b}", abit);
}