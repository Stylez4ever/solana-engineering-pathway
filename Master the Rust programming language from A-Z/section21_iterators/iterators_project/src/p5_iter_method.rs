fn practice_5() {
    let my_vector = vec![4, 8, 15, 16, 23, 42];
    //let iter = my_vector.iter();
    

    for number in &my_vector {
        println!("{:?}", number);
    }

    for number in my_vector.iter() {
        println!("{:?}", number);
    }
}
