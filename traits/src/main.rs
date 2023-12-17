struct Satellite {
    name: String,
    velocity: f64 // miles per second
}

struct SpaceStation {
    name: String,
    crew_size: u8,
    altitude: u32 // miles
}

struct SpaceJunk {
    orbit: f64,     // miles
    velocity: f64,  // miles per second
    mass:   f64     // kilograms
}

trait Description {
    fn describe(&self) -> String {
        String::from("there is lots of debris flying around the planet!")
    }
}

impl Description for Satellite {
    fn describe(&self) -> String {
        format!("the {} flying at {} miles per second", self.name, self.velocity)
    }
}

impl Description for SpaceStation {
    fn describe(&self) -> String {
        format!("the {} flying at altitude of {} miles with a crew of {}", self.name, self.altitude, self.crew_size)
    }
}

impl Description for SpaceJunk {
    
}
fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72
    };
    let iss = SpaceStation {
        name: String::from("International Space Station"),
        crew_size: 6,
        altitude: 254
    };
    let debris = SpaceJunk {
        orbit:      78.3,
        velocity:   832.6,
        mass:       23.7
    };

    println!("{}", hubble.describe());
    println!("{}", iss.describe());
    println!("{}", debris.describe());
}
