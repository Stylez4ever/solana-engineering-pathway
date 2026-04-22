use chrono::{TimeDelta};

fn main() {
    let five_seconds = TimeDelta::new(5, 0);
    println!("{:?}", five_seconds);

    let invalid = TimeDelta::new(5, 1_000_000_000);
    println!("{:?}", invalid);

    let negative_five_seconds = TimeDelta::new(-5, 0);
    println!("{:?}", negative_five_seconds);

    let five_minutes = TimeDelta::minutes(-5);
    println!("{:?}", five_minutes);

    let five_hours = TimeDelta::hours(5);
    println!("{:?}", five_hours);

    let five_days = TimeDelta::days(5);
    println!("{:?}", five_days);

    let five_weeks = TimeDelta::weeks(5);
    println!("{:?}", five_weeks);

    println!("{}", five_weeks.num_days());
    println!("{}", five_weeks.num_hours());
    println!("{}", five_weeks.num_minutes());

    let total_duration = five_weeks + five_days + five_hours + five_minutes;
    println!("{:?}", total_duration);

    println!("{} weeks, {} days, {} hours, {} minutes",
        total_duration.num_weeks(),
        total_duration.num_days(),
        total_duration.num_hours(),
        total_duration.num_minutes()
    )


}

