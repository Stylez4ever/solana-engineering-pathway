#[derive(Debug)]
struct Food {
    name: String,
}

#[derive(Debug)]
struct Restaurant {
    reservations: u32,
    has_mice_infestation: bool,
}

impl Restaurant {
    fn chef_special(self: &Restaurant) -> Option<Food> {

        if self.has_mice_infestation == true {
            Option::None
        } else if self.reservations < 12 {
            Option::Some(Food {name: String::from("Uni Sashimi")})
        } else {
            Option::Some(Food { name: String::from("Strip Steak") })
        }
    }

    fn deliver_burger(self: &Restaurant, address: &str) -> Result<Food, String> {
        if self.has_mice_infestation == true {
            Result::Err(String::from("Sorry, we have a mice problem"))
        } else if address.is_empty() {
            Result::Err(String::from("No delivery address specified"))
        } else {
            Result::Ok(Food { name: String::from("Beef burger") })
        }
    }
      
         
}


fn main() {

    let top_shit = Restaurant {
     reservations: 11,
     has_mice_infestation: true
    };

    println!("{:?}", top_shit.chef_special());
    println!("{:?}", top_shit.deliver_burger("123 Elm Street"));

    let dilo = Restaurant {
        reservations: 15,
        has_mice_infestation: false,
    };



    println!("{:?}", dilo.chef_special());
    println!("{:?}", dilo.deliver_burger(""));
    println!("{:?}", dilo.deliver_burger("mosvil"));



}



