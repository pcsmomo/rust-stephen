fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements
        .iter()
        .map(|el| el.to_uppercase())
        .collect::<Vec<_>>()
}

fn main() {
    let colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    let uppercased = to_uppercase(&colors);
    println!("{:#?}", uppercased);
}
