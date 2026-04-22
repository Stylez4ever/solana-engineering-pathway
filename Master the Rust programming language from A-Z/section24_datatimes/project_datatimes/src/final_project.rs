use chrono::prelude::*;

fn main() {
    let event_data = vec![
        (
            "2025**04**19 !! 16:00:00 -04:00",
            "Started Rust study session",
        ),


        ("2025**04**20 !! 08:05:30 -04:00", "Made breakfast"),


        ("ERR", "ERR"),


        ("2025**04**22 !! 22:10:45 -04:00", "Went to bed"),


        ("ERR", "ERR"),


        ("2025**04**25 !! 09:00:03 -04:00", "Resumed Rust study"),
    ];

    //let try_one = event_data
    //.into_iter()
    //.filter(|value| value == event_data.contains("2025")).collect::<Vec<(&str, &str)>>();

    let format = "%Y**%m**%d !! %H:%M:%S %z";

    let events = event_data
        .into_iter()
        .filter_map(|(timestamp, message)| {
            let parsed_datetime = DateTime::parse_from_str(timestamp, format);
            match parsed_datetime {
                Ok(datetime) => Some((datetime.with_timezone(&Utc), message)),
                Err(_) => None,
            }

        }).collect::<Vec<(DateTime<Utc>, &str)>>();

     let mut previous_event: Option<DateTime<Utc>> = None;   


    for (utc_datetime, message) in events {
        let display_time = utc_datetime.format("%Y-%m-%d %H:%M:%S");
        println!("Event time: {display_time}");
        println!("Event message: {message}");

        if let Some(previous_datetime) = previous_event {
            let difference = utc_datetime - previous_datetime;
            let hours = difference.num_hours();
            let minutes = difference.num_minutes() % 60;
            let seconds = difference.num_seconds() % 60;
            println!("Time since previous event: {}h {}m {}s", hours, minutes, seconds);
       }

        println!();
        previous_event = Some(utc_datetime);
    }    












}

