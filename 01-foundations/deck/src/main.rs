struct Deck {
    cards: Vec<String>
}

fn main() {
    let deck: Deck = Deck { cards: vec![] };
    // let deck: Deck = Deck { cards: Vec::new() }; // same as above

    println!("Heres your deck: {}", deck);
}
