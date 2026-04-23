use regex::Regex;

fn main() {

    // niddle
<<<<<<< HEAD
    let re = Regex::new(r"(?<count>\d+)\s(?<fruit>\w+)").unwrap();
    //heystack
    let text = "I have 2 apples and 10 bananas";
    let result = re.replace_all(text, "$fruit ($count)");
    println!("{}", result);
=======
    

    let re = Regex::new(r"\b[a-leading]").unwrap();
    //heystack
    let text = "My ZIP code is 90210. I am very rich";

    for data in re.find_iter(text) {
        println!("{:?}", data);
    }
>>>>>>> 28661da5dd0e1ca1a1f353a55b34bf51af4c3239
}
