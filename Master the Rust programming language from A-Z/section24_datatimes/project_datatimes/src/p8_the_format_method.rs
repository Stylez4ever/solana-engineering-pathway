use chrono::prelude::*;


fn main() {
    let utc_time = Utc::now();
    println!("{}", utc_time.format("%m-%d-%Y"));
    println!("{}", utc_time.format("%m/%d/%Y"));
    println!("{}", utc_time.format("%m-%d-%y"));
    println!("{}", utc_time.format("%b %d, %H:%M:%S"));
    println!("{}", utc_time.format("%A %I:%M %p %z"));
}
