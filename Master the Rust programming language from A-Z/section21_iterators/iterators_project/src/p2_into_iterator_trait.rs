use std::collections::HashMap;

fn practice_2() {
    let my_vector = vec![4, 8, 15, 16, 23, 42];
    let my_iterator = my_vector.into_iter();

    let my_vector = vec![false, true, true];
    let my_iterator = my_vector.into_iter();

    let mut my_hashmap = HashMap::new();
    my_hashmap.insert("boobs", 5);
    let my_iterator = my_hashmap.into_iter();

    // if a type can be used as the basis of creating an iterator such as a vector or a HashMap
    // they will implement the into iterator trait and then what we get back from that into_iterator
    // method call will be the type that implements the iterator trait itself
}
