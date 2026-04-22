use chrono::{TimeDelta, NaiveDate};
use std::ops::{Add, Sub};

fn main() {
    let birthday = NaiveDate::from_ymd_opt(1998, 3, 12).unwrap();
    println!("{}", birthday.add(TimeDelta::days(5)));

    println!("{}", birthday.add(TimeDelta::weeks(2) + TimeDelta::days(5)));

    println!("{}", birthday + TimeDelta::weeks(2) + TimeDelta::days(5));

    println!("{}", birthday.sub(TimeDelta::weeks(3)));

    println!("{}", birthday - TimeDelta::weeks(10));
}

