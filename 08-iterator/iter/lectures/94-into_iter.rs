fn move_elements(vec_a: Vec<String>, vec_b: &mut Vec<String>) {
    vec_a.into_iter().for_each(|el| vec_b.push(el));
}

fn main() {
    let colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    let mut destination = vec![];
    move_elements(colors, &mut destination);
    println!("{:#?}", destination);
}
