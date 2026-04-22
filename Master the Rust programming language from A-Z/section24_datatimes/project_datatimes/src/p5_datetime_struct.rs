use chrono::prelude::*;

fn main() {
    let system_time = Local::now();
    let utc_time = Utc::now();

    println!("{}", system_time.date_naive());
    println!("{}", utc_time.date_naive());
    println!("{}", system_time.time());
    println!("{}", utc_time.time());

    println!("{}", system_time.year());
    println!("{}", utc_time.year());
    println!("{}", system_time.month());
    println!("{}", utc_time.month());

    println!("{}", system_time.day());
    println!("{}", utc_time.day());

}

