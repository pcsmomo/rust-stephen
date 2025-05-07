
#[derive(Debug)]
struct Deck {
    cards: Vec<String>
}

fn main() {
    let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];
    let values = ["Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack", "Queen", "King"];

    let mut cards = vec![];

    for suit in suits {
        for value in values {
            let card = format!("{} of {}", value, suit);
            cards.push(card);
        }
    }

    let deck: Deck = Deck { cards };
    // let deck: Deck = Deck { cards: Vec::new() }; // same as above

    println!("Heres your deck: {:#?}", deck);
}
