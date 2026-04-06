fn practice_6() {
    let mut flavors = [
        String::from("Chocolate"),
        String::from("Vanilla"),
        String::from("Strawberry"),
    ];

    //let iterator = flavors.iter_mut();

    //for flavor in flavors.iter_mut() {
    //    flavor.push_str(" Ice Cream");
    //}

    for flavor in &mut flavors {
        flavor.push_str(" Ice Cream");
    }

    println!("{:?}", flavors);

    let mut school_grade = [85, 90, 72, 92];

    for grade in &mut school_grade {
        *grade = *grade - 2
    }
    println!("{:?}", school_grade);

    // OWNERSHIP
    // for value in collection
    // for value in collection.into-iter()


    // IMMUTABLE REFERENCES
    // for value in &collection
    // for value in collection.iter()


    // MUTABLE REFERENCES
    // for value in &mut collection
    // for value in collection.iter_mut()




}
