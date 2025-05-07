use rand::{rng, seq::SliceRandom};

// use rand::thread_rng;    // same as above
// use rand::seq::SliceRandom; // same as above

#[derive(Debug)]
struct Deck {
    cards: Vec<String>
}

// impl block is used to define methods for a struct,
impl Deck {

    // new is a method that returns a new instance of the Deck struct
    fn new() -> Self {  // without "$self", it is an associated function
        let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];
        let values = ["Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack", "Queen", "King"];

        let mut cards = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        // return Deck { cards };   // same as below
        Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut _rng = rng();
        self.cards.shuffle(&mut _rng);
    }
}

fn main() {
    let mut deck = Deck::new();

    deck.shuffle();

    println!("Heres your deck: {:#?}", deck);
}
