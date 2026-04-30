fn main() {
    let text = String::from("Hello");
    output_text(&text); // &String -> &str

    let my_box = Box::new(text);
    let value = &(*my_box)[..];
    output_text(value);
}

fn output_text(text: &str) {
    println!("{}", text)
}
