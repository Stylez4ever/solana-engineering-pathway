use std::collections::HashMap;

fn practice_7() {
    let mut todos = HashMap::new();
    todos.insert("Pick up groceries", false);
    todos.insert("Study Rust", true);
    todos.insert("Sleep", false);

    for (todo, completion_status) in &mut todos {
        //println!("Task: {}. Complete: {}", todo, completion_status);
        // dereference operator what we are saying is go to the memory that we are 
        // getting here and because we have a mutable reference. we have permission
        // to edit the value at that memory address
        *completion_status = true;

    }

    println!("{:?}", todos);




}
