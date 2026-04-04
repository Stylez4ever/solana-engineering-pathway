use section18_traits::lodging::{Accommodation, Description, AirBnB, Hotel};
use section18_traits::utlis;

fn main() {
    let mut hotel = Hotel::new(String::from("Tlou"));
    println!("{}", hotel.summarize());
    hotel.book("Thabang", 5);




    let mut airbnb = AirBnB::new("Tyson");
    println!("{}", airbnb.get_description());
    utlis::book_for_one_night(&mut airbnb, "Banafa");

    utlis::mix_and_match(&mut hotel, &mut airbnb, "Mazwu");

}





