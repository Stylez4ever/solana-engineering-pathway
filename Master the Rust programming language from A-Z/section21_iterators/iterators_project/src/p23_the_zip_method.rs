fn practice_23() {
    let first_names = ["Malestoko", "Bafana", "Khomotso"];
    let last_names = ["Masha", "Maja", "Mnisi"];

    let zip_it = first_names.iter().zip(last_names);
    
    for (maina, difane) in zip_it {
        println!("{maina} {difane}");
    }

    // create a vector of the full names using our basic method
    let complete_name: Vec<String> = first_names.iter().zip(last_names)
        .map(|(maina, difane)| format!("{maina} {difane}"))
        .collect();

    println!("{:?}", complete_name);
}
