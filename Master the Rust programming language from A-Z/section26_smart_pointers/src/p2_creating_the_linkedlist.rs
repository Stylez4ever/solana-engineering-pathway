#[derive(Debug)]
enum LinkedList<T> {
    Empty,
    Node { value: T, next: Box<LinkedList<T>>},
}

fn main() {
    let list = LinkedList::Node { 
        value: true, 
        next: Box::new(LinkedList::Node {
             value: false, next: Box::new(LinkedList::Node {
                 value: true, next: Box::new(LinkedList::Empty), 
                
                }) 
            }),
        };

    println!("{list:#?}");

    let final_song = LinkedList::Node { 
        value: String::from("Broski"), next: Box::new(LinkedList::Empty), };

    let middle_song = LinkedList::Node { 
        value: String::from("I'm not afraid"), 
        next: Box::new(final_song) };  

    let first_song = LinkedList::Node {
        value: String::from("One man can change the world"), 
        next: Box::new(middle_song) };   

    println!("{first_song:#?}");
}
