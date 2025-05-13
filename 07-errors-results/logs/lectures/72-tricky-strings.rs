fn string_test(a: String, b: &String, c: &str) {}

fn main() {
    string_test(
        "red".to_string(),
        &String::from("red"),
        String::from("red").as_str(),
    );
}
