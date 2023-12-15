fn main() {
    let outer_planet: String;
    {
        let mut inner_planet = String::from("Mercury");
        println!("inner_planet is {}", inner_planet);
        outer_planet = inner_planet.clone();
        println!("inner_planet is {}", inner_planet);
        inner_planet = String::from("Uranus");
        println!("inner_planet is {}", inner_planet);
    }
    println!("outer_planet is {}", outer_planet);
}