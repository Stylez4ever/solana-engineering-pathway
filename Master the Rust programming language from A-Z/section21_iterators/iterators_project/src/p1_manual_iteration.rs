fn practice_1() {
    let numbers = vec![4, 8, 15, 16, 23, 42];

    let mut current_index = 0;
    let final_index = numbers.len()- 1;

    loop {

        let index = numbers[current_index];
        println!("Index: {}", index);


        current_index = current_index + 1;
        if current_index > final_index {
            break;
        }
    }

    // simplifying the code and avoiding potential errors
    for number in numbers {
        println!("element: {}", number);
    }
}
