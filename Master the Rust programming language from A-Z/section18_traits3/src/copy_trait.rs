#[derive(Debug, Clone)]
struct Duration {
    hours: u32,
    minutes: u32,
    seconds: u32,
}

impl Duration {
    //constractor
    fn new(hours: u32, minutes: u32, seconds: u32) -> Duration {
        Duration { hours, minutes, seconds }
    }
}


//calling the copy trait from the Clone trait
impl Copy for Duration {}

fn copy_trait() {
//implementing the copy Trait
    let one_hour = Duration::new(1, 2, 5);
    let another_hour = one_hour;

    println!("{:?}", one_hour); 
}
