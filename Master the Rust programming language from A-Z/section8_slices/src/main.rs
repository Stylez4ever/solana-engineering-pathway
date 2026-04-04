fn main() {
    let cereals = ["Cookie Crisp", "Cinnamon Toast Crunch", "Frosted Flakes", "Cocoa Puffs", "Captain Crunch"];
    let first_two = &cereals[..2];

    println!("{first_two:?}");

    let mid_three = &cereals[1..4];
    println!("{mid_three:?}");

    let last_three = &cereals[2..];
    println!("{last_three:?}");

    let cookie_crisp = &cereals[0];
    let cookie = &cookie_crisp[0..6];

    println!("{cookie:?}");

    let cocoa_puffs = &cereals[3].to_string();
    let puffs = &cocoa_puffs[6..];
    println!("{puffs:?}");



    
}
