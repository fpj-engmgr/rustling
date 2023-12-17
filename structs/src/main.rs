#[derive(Debug)]
#[derive(Clone)]

struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64
}

impl Shuttle {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn read_fuel(&self) -> f64 {
        self.propellant
    }

    fn add_fuel(&mut self, gallons: f64) {
        self.propellant += gallons;
    }
    fn new(name: &str) -> Shuttle {
        Shuttle {
            name: String::from(name),
            crew_size: 7,
            propellant: 0.0
        }
    }
}

fn main() {
    let mut vehicle  = Shuttle::new("Endeavour");
    let mut vehicle2 = Shuttle::new("Discovery");

    println!("name is {}", vehicle.name);

    println!("vehicle2 is {:?}", vehicle2);

    vehicle.name = String::from("Atlantis");
    println!("vehicle is {:?}", vehicle);
    println!("vehicle2 is {:?}", vehicle2);

    println!("vehicle1 name is {}\nvehicle2 name is {}",vehicle.get_name(),vehicle2.get_name());

    println!("original propellant for {} is {}", vehicle.get_name(), vehicle.read_fuel());
    vehicle.add_fuel(120064.0);
    println!("updated propellant for {} is {}", vehicle.get_name(), vehicle.read_fuel());
}