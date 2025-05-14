fn print_elements(elements: &[String]) {
    // for element in elements {
    //     println!("{}", element);
    // }
    elements
        .iter()
        .map(|el| format!("{} {}", el, el))
        .for_each(|el| println!("{}", el));
}

fn shorten_strings(elements: &mut Vec<String>) {}

fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    // print_elements(&colors);
    shorten_strings(&mut colors);
}
