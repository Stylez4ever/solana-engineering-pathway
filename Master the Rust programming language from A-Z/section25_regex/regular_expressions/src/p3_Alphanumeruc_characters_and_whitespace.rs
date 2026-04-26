use regex::Regex;

fn practice_3() {
    // niddle
    // \d = Any numeric digit at all
    // \D = Any character in the seach string that is not a digit
    // \w = Only charactor and digits
    // \W = Any match anywhere for anything that is not a charactor and not a digit
    // \s = Mach space or whitespace
    // \S = Match Anything that is not whitespace
    let re = Regex::new(r"\s").unwrap();
    //heystack
    let text = "My ZIP code is 90210. I am very rich";

    for data in re.find_iter(text) {
        println!("{:?}", data);
    }
}
