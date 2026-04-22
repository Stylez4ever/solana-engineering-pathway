use chrono::prelude::*;
use chrono_tz::Africa::Johannesburg;

fn main() {
    let local_time = Local::now();
    let utc_time = local_time.with_timezone(&Johannesburg);

    println!("{}", local_time);
    println!("{}", utc_time);
}

