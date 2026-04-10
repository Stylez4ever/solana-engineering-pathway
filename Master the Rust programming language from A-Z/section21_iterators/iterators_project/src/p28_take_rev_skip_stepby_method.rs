fn practice_28() {
    let fifty_numbers = 1..=50;

    // take method
    for number in fifty_numbers.clone().take(15) {
        print!("{number}/");
    }

    println!();

    // take method with a rev method
    for number in fifty_numbers.clone().rev().take(15) {
        print!("{number}/");
    }

    println!();

    // take method with the skip method
    for number in fifty_numbers.clone().skip(5).take(15) {
        print!("{number}/");
    }

    println!();

    // take and skip method with the step_by method
    for number in fifty_numbers.take(15).skip(5).step_by(2) {
        print!("{number}/");
    }
}
