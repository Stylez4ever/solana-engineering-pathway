use regex::Regex;

fn practice_2() {
    // niddle
    // \d = Any numeric digit at all
    // \D = Any character in the seach string that is not a digit
    let re = Regex::new(r"\D").unwrap();
    //heystack
    let text = "My ZIP code is 90210. I am very rich";

    for data in re.find_iter(text) {
        println!("{:?}", data);
    }
}
