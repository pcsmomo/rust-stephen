fn string_test(a: String, b: &String, c: &str) {
    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
}

fn main() {
    string_test(
        "red".to_string(),
        &String::from("red"),
        String::from("red").as_str(),
        // &String::from("blue"), // automaticaly turn into &str
    );
}
