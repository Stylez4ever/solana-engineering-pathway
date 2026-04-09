fn practice_25() {
    let earning = [4, 7, 9, 13];

    let sum = earning
        .into_iter()
        .reduce(|total, current| total + current);

    println!("{:?}", sum);


    let address_portions = [
        String::from("6000 Marokolong"),
        String::from("Hammaskraal"),
        String::from("Pretoria"),
    ];

    // 6000 Marokolong, Hammaskraal, Pretoria
    println!("{}", address_portions.join(", "));

    let address = address_portions
        .into_iter()
        .reduce(|mut accumulator, portion| {
            accumulator.push_str(", ");
            accumulator.push_str(&portion);
            accumulator
        });

    println!("{address:?}");
}
