
enum Location {
    Unknown,
    Anonymous,
    Known(f64, f64)
}

impl Location {
    fn display(&self) {
        match *self {
            Location::Unknown       => println!("This is Terra Incognita"),
            Location::Anonymous     => println!("I'm not supposed to tell you..."),
            Location::Known(x, y)   => println!("Set your GPS to {} latitude and {} longitude", x, y)
        }
    }
}

fn main() {
    let address = Location::Unknown;
    address.display();
    let address = Location::Anonymous;
    address.display();
    let address = Location::Known(28.608295, -80.604177);
    address.display();
}
