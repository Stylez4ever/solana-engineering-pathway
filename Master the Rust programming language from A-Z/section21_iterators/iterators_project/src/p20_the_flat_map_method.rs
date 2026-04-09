fn practice_20() {
    let attendees = [
        "Bob, Mary, Kevin",
        "Mike, Robbie, Matt, Austin",
        "Piers, Liam",
    ];

    let groups: Vec<&str> = attendees.iter().map(|group| group.split(", ")).flatten().collect();
    println!("{:?}", groups);

    // flat_map methods
    let groups: Vec<&str> = attendees.iter().flat_map(|group| group.split(", ")).collect();
    println!("{:?}", groups);
}
