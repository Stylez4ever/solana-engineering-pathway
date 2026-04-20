use rand::{Rng, RngExt, rng, seq::SliceRandom};

fn practice_3() {
    let mut my_rng = rng();
    let mut candies = vec![
        "Sour Patch Kids",
        "Kit kat",
        "Twix",
        "Snickers",
        "Starburst",
    ];
    candies.shuffle(&mut my_rng);

    println!("{:?}", candies);
}