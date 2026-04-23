use regex::Regex;

fn practice_4() {

    // niddle
    // \b = looks for a word boundary.
    // r"\bc" and r"e/b"
    let re = Regex::new(r"\b").unwrap();
    //heystack
    let text = "My ZIP code is 90210. I am very rich";

    for data in re.find_iter(text) {
        println!("{:?}", data);
    }
}