use rand::{Rng, RngExt, rng, seq::SliceRandom};

#[derive(Clone, Copy, Debug)]
enum Suit {
    Clubs,
    Spades,
    Hearts,
    Diamonds,
}


#[derive(Clone, Copy, Debug)]
enum Rank {
    Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten,
    Jack, Queen, King, Ace, Joker,
}


#[derive(Debug)]
struct Card {
    rank: Rank,
    suit: Option<Suit>
}

#[derive(Debug)]
struct Deck {
    cards: Vec<Card>
}

impl Deck {
    fn new() -> Self {
        let suits = [Suit::Clubs, Suit::Spades, Suit::Hearts, Suit::Diamonds];
        let ranks = [ 
            Rank::Two, 
            Rank::Three, 
            Rank::Four, 
            Rank::Five, 
            Rank::Six, 
            Rank::Seven, 
            Rank::Eight, 
            Rank::Nine, 
            Rank::Ten,
            Rank::Jack, 
            Rank::Queen, 
            Rank::King, 
            Rank::Ace, 
            ];

            let mut cards = Vec::with_capacity(52);

            for suit in suits.into_iter() {
                for rank in ranks.into_iter() {
                    cards.push(Card {
                        suit: Some(suit),
                        rank,
                    });
                }
            }

            Self { cards}

                     

    }

    fn shuffle(&mut self) {
        let mut my_rng = rng();
        self.cards.shuffle(&mut my_rng);
    }

    fn insert_jokes(&mut self) {
        let mut my_rng = rng();
        for _ in 0..2 {
            let random_index = my_rng.random_range(0..self.cards.len());
            let joker_card = Card {
                suit: None,
                rank: Rank::Joker,
            };
            self.cards.insert(random_index, joker_card);
        }
    }

    fn delete_random_card(&mut self) {
        let mut my_rng = rng();
        let should_delete_card = my_rng.random_bool(0.65);
        if should_delete_card {
            let random_index = my_rng.random_range(0..self.cards.len());
            self.cards.remove(random_index);
        }
    }


}

fn main() {
    let mut on_deck = Deck::new();
    on_deck.shuffle();
    on_deck.insert_jokes();

    for _ in 0..10 {
        on_deck.delete_random_card();
    }

    println!("{:#?}", on_deck.cards.len());
}