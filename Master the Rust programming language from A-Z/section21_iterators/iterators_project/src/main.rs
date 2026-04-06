fn main() {
    let names = [
        String::from("Jimmy"),
        String::from("Cleveland"),
        String::from("Boris"),
    ];

    let name_length: Vec<_> = names.iter()
    .map(|name| name.to_lowercase())
    .map(|name| name.replace("i", "@@"))
    .map(|name| name.len()).collect();

    println!("{:?}", name_length);


}
