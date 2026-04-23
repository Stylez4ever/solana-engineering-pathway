use regex::Regex;

fn main() {

    // niddle
    let re = Regex::new(r"\d+$").unwrap();
    //heystack
    let text = "/users/1/posts/353";


    // niddle
    let re_1 = Regex::new(r"^/v\d+").unwrap();
    //heystack
    let text_1 = "/v3/items/v2/prices/v9";




    for data in re_1.find_iter(text_1) {
        println!("{:?}", data);
    }
}
