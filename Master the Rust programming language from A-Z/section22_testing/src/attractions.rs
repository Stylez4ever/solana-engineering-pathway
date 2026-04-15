trait TicketSeller {
    fn sell_ticket(&mut self);
}


#[derive(Debug, PartialEq, Eq)]
struct Museum {
    painting: Vec<String>,
    revenue: u32,
}

impl Museum {
    const MAXIMUM_CAPACITY: usize = 3;


    fn new() -> Self {
        Self {
            painting: vec![],
            revenue: 0,
        }
    }

    fn buy_painting(&mut self, new_painting: &str) {
        if self.painting.len() >= Self::MAXIMUM_CAPACITY {
            panic!("Museum does not have storage space for another painting");
        }


        self.painting.push(new_painting.to_string());
    }


    fn has_impressive_collection(&self) -> bool {
        self.painting.len() > 2
    }
}

impl TicketSeller for Museum {
    fn sell_ticket(&mut self) {
    self.revenue += 25;
    }
}


#[derive(Debug)]
struct MovieTheater {
    movies: Vec<String>,
    sales: u32,
}

impl MovieTheater {
    fn new() -> Self {
        Self {
            movies: vec![],
            sales: 0,
        }
    }

    fn add_movies(&mut self, movie: &str) {
        self.movies.push(movie.to_string());
    }


}

impl TicketSeller for MovieTheater {
        fn sell_ticket(&mut self) {
        self.sales += 15;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};


    #[test]
    fn museum_sells_ticket_to_increase_revenue() -> Result<(), String> {
        let mut museum = Museum::new();
        museum.sell_ticket();

        if museum.revenue == 25 {
            Result::Ok(())
        } else {
            Result::Err(String::from("The revenue from selling 1 ticket did not match expectation."))
        }
        
    }

    #[test]
    #[ignore]
    fn museum_sells_ticket_to_increase_revenue_1() {
        let mut museum = Museum::new();
        museum.sell_ticket();
        assert_ne!(museum.revenue, 0);
    }

    #[test]
    fn museum_can_sell_multiple_ticket() {
        let mut museum = Museum::new();
        museum.sell_ticket();
        museum.sell_ticket();
        assert_eq!(museum.revenue, 50);
    }

    #[test]
    fn museum_can_have_impressive_art_collection() -> Result<(), String> {
        let mut museum = Museum::new();
        museum.buy_painting("Goku spirit boom");
        museum.buy_painting("Vegita turning super");
        museum.buy_painting("Gohun one hand kamekamea");

        if museum.has_impressive_collection() {
            Result::Ok(())
        } else {
            Result::Err(String::from("the museum did not have an impressive collection despite having more than 2 painting."))
        }
    
    }

    #[test]
    #[ignore]
    fn hashmaps() {
        let mut one = std::collections::HashMap::new();
        one.insert("A", 4);
        one.insert("B", 3);

        let mut two = std::collections::HashMap::new();
        two.insert("B", 3);
        two.insert("A", 4);

        assert_eq!(one, two);
    }

    #[test]
    fn new_museums_are_equal () {
        let museum_1 = Museum::new();
        //museum_1.sell_ticket();

        let museum_2 = Museum::new();

        assert_eq!(museum_1, museum_2, "two new Museum instances were not found to be equal: {museum_1:?} // {museum_2:?}");

    }


    #[test]
    #[should_panic(expected = "storage space")]
    fn museum_prohibits_adding_paintings_when_capacity_has_been_reached() {
        let mut museum = Museum::new();
        museum.buy_painting("Goku spirit boom");
        museum.buy_painting("Vegita turning super");
        museum.buy_painting("Gohun one hand kamekamea");
        museum.buy_painting("Goku go Ultra instint");

    }

    #[test]
    #[ignore]
    fn print_success() {
        println!("Success inside the function");
        assert!(true)
    }

    #[test]
    #[ignore]
    fn print_failure() {
        println!("Failure inside the function");
        assert!(false)
    }
    
}



