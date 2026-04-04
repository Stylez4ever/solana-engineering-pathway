// The PartialEq trait compares all the fields
//#[derive(PartialEq ,Debug)]
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
    //equal method which which recieves two reference and compare with they equal and return a bool 
    // with selected fields
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.destrination == other.destrination
    }
}

fn partial_eq() {
    // Implementing the PartialEq Trait for Struct
    let going_home = Flight::new("Middelburg", "stoffburg", "08:00");
    let going_to_granny = Flight::new("Middelburg", "stoffburg", "08:00");
    let going_to_school = Flight::new("Stoffburg", "Middelburg", "08:00");

    println!("{}", going_home == going_to_granny);
    println!("{}", going_home.eq(&going_to_granny));
    println!("{}", going_home.ne(&going_to_school));

 
}
