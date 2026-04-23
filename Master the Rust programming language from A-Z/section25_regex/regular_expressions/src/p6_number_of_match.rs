use regex::Regex;

fn main() {

    // niddle
    

    let re = Regex::new(r"\d{3}\s\d{3}\s\d{4}").unwrap();
    //heystack
    let text = "Hey, this is Tyson. Your program sucks. I can be reached 
        at 555  123    4567 and I'd appreciate a call tomorrow";

    for data in re.find_iter(text) {
        println!("{:?}", data);
    }
}
