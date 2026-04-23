use regex::Regex;

fn practice_5() {

    // niddle
    let re = Regex::new(r"\b[a-leading]").unwrap();
    //heystack
    let text = "My ZIP code is 90210. I am very rich";

    for data in re.find_iter(text) {
        println!("{:?}", data);
    }
}
