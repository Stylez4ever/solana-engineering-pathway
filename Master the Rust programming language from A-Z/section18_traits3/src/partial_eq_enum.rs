//#[derive(PartialEq)]

enum Musician {
    SingerSongwriter(String),
    Band(u32),
}

use Musician::{SingerSongwriter, Band};

impl PartialEq for Musician {
    fn eq(&self, other: &Self) -> bool {
        match self {
            SingerSongwriter(name) => match other {
                Self::SingerSongwriter(other_name) => name == other_name,
                Band(_) => false
            },
            Band(member) => match other {
                SingerSongwriter(_) => false,
                Band(other_member) => member == other_member,
            }
        }
    }
}

fn partial_eq_enum() {
    // Implementing the PartialEq Traits for enums
    let rustin_bieber = SingerSongwriter("Rustin".to_string());
    let rustin_timber = SingerSongwriter("Rustin".to_string());
    let holly = SingerSongwriter("Holly".to_string());

    let rust_no_one = Band(5);
    let unrustworthy = Band(4);
    let rust_for_vengeance = Band(5);

    println!("{}", rustin_bieber == holly);
    println!("{}", rustin_bieber == rustin_timber);
    println!("{}", rustin_bieber == rust_no_one);

    println!("{}", rust_no_one == unrustworthy);
    println!("{}", rust_no_one == rust_for_vengeance);
}
