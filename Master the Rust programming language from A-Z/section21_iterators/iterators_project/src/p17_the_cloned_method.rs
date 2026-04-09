fn practice_17() {
    let teas = [
        String::from("Hot Earl Grey"),
        String::from("Iced Green"),
        String::from("Hot rooibos"),
    ];

    // iterator has &string -->(cloned()) --> String
    let more_teas: Vec<String> = teas.iter().cloned().collect();
    println!("{:?}", more_teas);

    let filter_teas_1: Vec<String> = teas.iter().filter(|tea| tea.contains("Hot")).cloned().collect();
    println!("{:?}", filter_teas_1);
}
