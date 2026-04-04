

fn main() {
    let v = vec![1, 2, 3];
    let v1 = vec![1, 2, 3, 4, 5];
    //println!("{:?}", v);
    //let result = double_the_length(&v);
    //println!("{:?}", result);

    //let dv = &v[2..];
    //println!("{:?}", dv);

    let result_2 = last_two(&v1);
    println!("{:?}", result_2);

    let result_3 = first_five("refrigerator", "Hello");
    println!("{:?}", result_3);

    let content = find_string_that_content("programming", "dining", "gram");
    println!("{}", content);

    

    

    
}

// Does this function need lifetime annotations?
// Explain why or why not.
// answer is no becease the lifetime of the vector ref ends in the function
// correction
fn double_the_length<'a>(value: &'a Vec<i32>) -> usize {
    let double = value.len() * 2;
    double
}


// Does this function need lifetime annotations?
// Explain why or why not.
// no, the is no new data
fn last_two<T>(value: &[T]) -> &[T] {
    let two_from_the_end = value.len() - 2;
    &value[two_from_the_end..]

}

// Does this function need lifetime annotations?
// Explain why or why not.
// yes, becease we have two parameters with the same type as the return type so rust will
// need us to the it which will is for the return
fn first_five<'a, 'b>(text: &'a str, announcemnt: &'b str) -> &'a str {
    println!("{}", announcemnt);
    &text[..5]
}

fn find_string_that_content<'a, 'b >(first: &'a str, second: &'a str, target: &'b str) -> &'a str {
    if first.contains(target) {
        first
    } else {
        second
    }
}



