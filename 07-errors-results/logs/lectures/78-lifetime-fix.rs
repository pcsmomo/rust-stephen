use std::fs;

fn extract_error(text: &str) -> Vec<String> {
    let split_text = text.split("\n");

    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line.to_string());
        }
    }

    results
}

fn main() {
    let mut error_logs = vec![];

    match fs::read_to_string("logs.txt") {
        Ok(text_that_was_read) => {
            error_logs = extract_error(text_that_was_read.as_str());
        }
        Err(why_this_failed) => {
            panic!("Failed to read file: {}", why_this_failed);
        }
    }

    println!("{:#?}", error_logs);
}
