use std::{collections::HashMap, fmt::format};

trait Accommodation {
    fn get_description(&self) -> String;
    fn book(&mut self, name: &str, night: u32);
}


#[derive(Debug)]
struct Hotel {
    name: String,
    reservations: HashMap<String, u32>,
}

impl Hotel {
    fn new(name: &str) -> Hotel {
        Hotel {
            name: name.to_string(),
            reservations: HashMap::new(),
        }
    }
}

impl Accommodation for Hotel {
    fn get_description(&self) -> String {
        format!("{} is the pinnacle of luxury", self.name)
    }

    fn book(&mut self, name: &str, night: u32) {
        self.reservations.insert(name.to_string(), night);
    }
}

#[derive(Debug)]
struct AirBnB {
    host: String,
    guest: Vec<(String, u32)>,
}

impl AirBnB {
    fn new(host: &str) -> AirBnB {
        AirBnB {
            host: host.to_string(),
            guest: vec![],
        }
    }
}

impl Accommodation for AirBnB {
    fn get_description(&self) -> String {
        format!("Please enjoy {}'s apartment", self.host)
    }

    fn book(&mut self, name: &str, night: u32) {
        self.guest.push((name.to_string(), night));
    }
}


fn practice_1() {
    let mut hotel = Hotel::new("The Tlou");
    println!("{}", hotel.get_description());
    hotel.book("khomotxo", 6);
    println!("{:#?}", hotel);
    

    let mut airbnb = AirBnB::new("Tyson");
    println!("{}", airbnb.get_description());
    airbnb.book("Khomotxo", 8);
    println!("{:#?}", airbnb);
}
