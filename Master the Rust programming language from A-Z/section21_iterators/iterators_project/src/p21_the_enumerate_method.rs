fn practice_21() {
    let applicants = vec!["Rob", "Bob", "Cob", "Alex", "Piers", "John", "Ben"];

    let winners: Vec<&str> = applicants
        .into_iter()
        .enumerate()
        .filter_map(|(index, applicants)| {
            if index % 3 == 0 {
                Some(applicants)
            } else {
                None
            }
        }).collect();

    println!("{:?}", winners);
}
