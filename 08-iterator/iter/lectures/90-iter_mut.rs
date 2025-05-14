fn shorten_strings(elements: &mut [String]) {
    elements.iter_mut().for_each(|el| el.truncate(1));
}

fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    shorten_strings(&mut colors[1..3]);
    println!("{:#?}", colors);
}
