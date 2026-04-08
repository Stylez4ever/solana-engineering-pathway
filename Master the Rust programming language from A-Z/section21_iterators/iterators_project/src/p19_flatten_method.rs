fn practice_19() {
    let spreadsheet = vec![
        [100, 200, 300],
        [330, 600, 900],
        [154, 298, 323],
    ];

    let value: Vec<i32> = spreadsheet.into_iter().flatten().collect();
    println!("{:?}", value);
}
