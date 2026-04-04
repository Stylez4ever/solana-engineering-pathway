use std::{collections::HashMap};
use std::fmt::Display;

trait Accommodation {
    fn book(&mut self, name: &str, night: u32);
}

trait Description {
    fn get_description(&self) -> String {
        String::from("A wonderful place to stay")
    }
}


#[derive(Debug)]
struct Hotel<T> {
    name: T,
    reservations: HashMap<String, u32>,
}

impl<T> Hotel<T> {
    fn new(name: T) -> Hotel<T> {
        Hotel {
            name,
            reservations: HashMap::new(),
        }
    }
}

impl<T: Display> Hotel<T> {
    fn summarize(&self) -> String {
        format!("{}: {}", self.name, self.get_description())
    }
}


impl<T> Accommodation for Hotel<T> {
    //fn get_description(&self) -> String {
    //    format!("{} is the pinnacle of luxury", self.name)
    //}

    fn book(&mut self, name: &str, night: u32) {
        self.reservations.insert(name.to_string(), night);
    }
}

impl<T> Description for Hotel<T> {}

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
    fn book(&mut self, name: &str, night: u32) {
        self.guest.push((name.to_string(), night));
    }
}

impl Description for AirBnB {
    fn get_description(&self) -> String {
        format!("Please enjoy {}'s apartment", self.host)
    }

}




fn practice_6() {
    let hotel1 = Hotel::new("Tlou");
    println!("{}", hotel1.summarize());

    let hotel2 = Hotel::new("ke Thaba");
    println!("{}", hotel2.summarize());


    

    
}



fn book_for_one_night<T: Accommodation + Description>(entity: &mut T, guest: &str) {
    entity.book(guest, 1);

}

fn mix_and_match<T, U>(first: &mut T, second: &mut U, guest: &str)
where T: Accommodation + Description,
      U: Accommodation,
{
    first.book(guest, 1);
    first.get_description();


    second.book(guest, 1);
}

fn choose_best_place_to_stay() -> impl Accommodation + Description {
    Hotel::new("The Tlou")
}
