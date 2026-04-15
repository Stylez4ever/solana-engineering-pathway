#[derive(Debug, PartialEq, Eq)]
struct Museum {
    painting: Vec<String>,
    revenue: u32,
}

impl Museum {
    fn new() -> Self {
        Self {
            painting: vec![],
            revenue: 0,
        }
    }

    fn buy_painting(&mut self, new_painting: &str) {
        self.painting.push(new_painting.to_string());
    }

    fn sell_ticket(&mut self) {
        self.revenue += 25;
    }

    fn has_impressive_collection(&self) -> bool {
        self.painting.len() > 2
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};


    #[test]
    fn museum_sells_ticket_to_increase_revenue() {
        let mut museum = Museum::new();
        museum.sell_ticket();
        assert_eq!(museum.revenue, 25);
    }

    #[test]
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
    fn museum_can_have_impressive_art_collection() {
        let mut museum = Museum::new();
        museum.buy_painting("Goku spirit boom");
        museum.buy_painting("Vegita turning super");
        museum.buy_painting("Gohun one hand kamekamea");
        assert!(museum.has_impressive_collection());
    }

    #[test]
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

        assert_eq!(museum_1, museum_2);

    }
}



