fn main() {
    let is_concert = true;
    let is_event = is_concert;
    // Will Rust move ownership? No, because bool is on the stack the copy trait will be used to make a copy of
    // of is_concert and store the copy in is_event

    println!("{is_concert}");
    println!("{is_event}");

    let sushi = "fish";
    let dinner = sushi;
    // Will Rust move ownership? No, because &str is on the stack the copy trait will be used to make a copy of
    // of sushi and store the copy dinner

    println!("{sushi}");
    println!("{dinner}");

    let sushi_1 = String::from("Salom");
    let dinner_1 = sushi_1;
    // Will Rust move ownership? Yes, because the String is on the head it will move ownership from sushi_1 to dinner_1

    //println!("{sushi_1}");
    println!("{dinner_1}");

   // eat_meal(dinner_1); // the movement of the Salom is the sushi_1 will pass ownership to dinner_1 and when the function
    // eat_meal is called dinner_1 is passed as the argument as a String and meal takes ownership of the value of dinner
    // which will be used in the function
    let dinner_1 = eat_meal(dinner_1);


}


fn eat_meal(mut meal: String) -> String {
    meal.clear();
    meal
}

