use regex::Regex;

fn main() {

    // niddle
    let re = Regex::new(r"(?<street_number>\d+)(.+?)(?<state>\w{2})").unwrap();
    //heystack
    let text = "123 Elm Street, Palm Spring, CA";
    let captures = re.captures(text).unwrap();
    println!("{}", &captures["street_number"]);
    println!("{}", &captures["state"]);
    //println!("{}", &captures[1]);
    //println!("{}", &captures[2]);
    //println!("{}", &captures[3]);
    

}
