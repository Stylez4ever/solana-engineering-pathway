use chrono::NaiveDate;

fn main() {
    let birthday = NaiveDate::from_ymd(1998, 4, 12);
    println!("{:?}", birthday);

    let my_birthday = "1998-04-15";
    let my_birthday = my_birthday
        .parse::<NaiveDate>()
        .expect("Unable to parse NaiveDate from string");

    println!("{:?}", my_birthday);

}

