use std::{collections::HashMap};

trait Accommodation {
    fn get_description(&self) -> String {
        String::from("A wonderful place to stay")
    }
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

    fn summarize(&self) -> String {
        format!("{}: {}", self.name, self.get_description())
    }
}

impl Accommodation for Hotel {
    //fn get_description(&self) -> String {
    //    format!("{} is the pinnacle of luxury", self.name)
    //}

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




fn practice_2() {
    let mut hotel = Hotel::new("The Tlou");
    book_for_one_night(&mut hotel, "Thabang");
    println!("{:#?}", hotel);
    

    let mut airbnb = AirBnB::new("Tyson");
    book_for_one_night(&mut airbnb, "Thabang");
    println!("{:#?}", airbnb);
    
}



fn book_for_one_night(entity: &mut impl Accommodation, guest: &str) {
    entity.book(guest, 1);

}
