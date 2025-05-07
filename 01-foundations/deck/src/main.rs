
#[derive(Debug)]
struct Deck {
    cards: Vec<String>
}

fn main() {
    let suits = vec!["Hearts", "Diamonds", "Clubs", "Spades"];
    let values = vec!["Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack", "Queen", "King"];

    let cards = vec![];

    for suit in suits {
        for value in values {
            let card = format!("{} of {}", value, suit);
            cards.push(card);
        }
    }

    let deck: Deck = Deck { cards: vec![] };
    // let deck: Deck = Deck { cards: Vec::new() }; // same as above

    println!("Heres your deck: {:?}", deck);
}
