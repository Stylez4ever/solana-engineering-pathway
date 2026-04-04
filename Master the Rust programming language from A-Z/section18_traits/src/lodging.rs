use std::{collections::HashMap};
use std::fmt::Display;

pub trait Accommodation {
    fn book(&mut self, name: &str, night: u32);
}

pub trait Description {
    fn get_description(&self) -> String {
        String::from("A wonderful place to stay")
    }
}


#[derive(Debug)]
pub struct Hotel<T> {
    name: T,
    reservations: HashMap<String, u32>,
}

impl<T> Hotel<T> {
    pub fn new(name: T) -> Hotel<T> {
        Hotel {
            name,
            reservations: HashMap::new(),
        }
    }
}

impl<T: Display> Hotel<T> {
    pub fn summarize(&self) -> String {
        format!("{}: {}", self.name, self.get_description())
    }
}


impl<T> Accommodation for Hotel<T> {
    fn book(&mut self, name: &str, night: u32) {
        self.reservations.insert(name.to_string(), night);
    }
}

impl<T> Description for Hotel<T> {}

#[derive(Debug)]
pub struct AirBnB {
    host: String,
    guest: Vec<(String, u32)>,
}

impl AirBnB {
    pub fn new(host: &str) -> AirBnB {
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