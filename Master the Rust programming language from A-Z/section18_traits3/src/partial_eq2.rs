// The PartialEq trait compares all the fields
// #[derive(PartialEq ,Debug)]

struct BusTrip {
    origin: String,
    destrination: String,
    time: String,
}

impl BusTrip {
    //constractor
    fn new(origin: &str, destination: &str, time: &str) -> Flight {
        Flight {
            origin: origin.to_string(),
            destrination: destination.to_string(),
            time: time.to_string(),
        }
    }
}

struct Flight {
    origin: String,
    destrination: String,
    time: String,
}

impl Flight {
    //constractor
    fn new(origin: &str, destination: &str, time: &str) -> Flight {
        Flight {
            origin: origin.to_string(),
            destrination: destination.to_string(),
            time: time.to_string(),
        }
    }
}

impl PartialEq for Flight {
    // equal method which which recieves two reference and compare with they equal and return a bool 
    // with selected fields
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.destrination == other.destrination
    }
}

impl PartialEq<BusTrip> for Flight {
    // equal method which which recieves two reference and compare with they equal and return a bool 
    // with selected fields
    fn eq(&self, other: &BusTrip) -> bool {
        self.time == other.time
    }
        
    
}

fn partial_eq_2() {
    // Defining equality for different type
    let going_home = Flight::new("Middelburg", "stoffburg", "08:00");
    let going_to_granny = BusTrip::new("Middelburg", "Burgersfort", "08:00");
    
    println!("{}", going_home == going_home);
    println!("{}", going_home == going_to_granny);
    println!("{}", going_to_granny == going_home);
    println!("{}", going_to_granny == going_to_granny);

}
