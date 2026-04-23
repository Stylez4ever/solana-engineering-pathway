use regex::Regex;

fn practice_1() {
    // niddle
    let re = Regex::new(r"ue").unwrap();
    //heystack
    let text = "my movie 🍿 queue";

    for data in re.find_iter(text) {
        println!("{} {} {}", data.start(), data.end(), data.as_str());
    }



    //match re.find(text) {
    //   Some(data) => {
    //    println!("{} {} {}", data.start(), data.end(), data.as_str());
    //}
    //    None => println!("No matches found bitch"),
    //}

   // match re.find(text) {
   //     Some(data) => {
   //         println!("{data:?}")
   //     }
   //     None => println!("No matches found bitch"),
   // }
}
