fn practice_11() {

    let numbers = vec![4, 8, 15, 16, 23, 42];

    //let my_iterator = numbers.iter();
    //let squares = 
    //my_iterator.map(|number: &i32| number.pow(2));
    //for number in squares {
    //    println!("Square: {number}")
    //}

    //simplified code
    // when using the map method it needs you to explicitly call the iterator method (iter(),
    // into_iter(), iter_mut())
    for number in numbers.iter().map(|number: &i32| number.pow(2)) {
        println!("Square: {number}")
    }
}

