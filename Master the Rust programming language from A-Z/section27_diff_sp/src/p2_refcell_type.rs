use std::cell::RefCell;

#[derive(Debug)]
struct ConcertTicket {
    section: String,
    seat: String,
    scanned: bool
}

impl ConcertTicket {
    fn new(section: String, seat: String) -> Self {
        Self { section, seat, scanned: false }
    }
}

fn main() {
    let ticket = RefCell::new(ConcertTicket::new(String::from("A"), String::from("5")));
    println!("{ticket:#?}");

    {
        let mut one = ticket.borrow_mut();
    }

    println!("{}", ticket.borrow().seat);



    ticket.borrow_mut().seat = String::from("K");
    println!("{}", ticket.borrow().seat);
;
}
