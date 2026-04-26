use regex::Regex;

fn main() {

    // niddle
    let re = Regex::new(r"(?<count>\d+)\s(?<fruit>\w+)").unwrap();
    //heystack
    let text = "I have 2 apples and 10 bananas";
    let result = re.replace_all(text, "$fruit ($count)");
    println!("{}", result);
}
