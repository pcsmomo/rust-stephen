fn explode(elements: &[String]) -> Vec<Vec<String>> {
    elements
        .iter()
        .map(|el| el.chars().map(|c| c.to_string()).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn main() {
    let colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    let exploded = explode(&colors);
    println!("{:#?}", exploded);
}
