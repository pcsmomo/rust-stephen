use std::fs;

fn main() {
    match fs::read_to_string("logs.txt") {
        Ok(text_that_was_read) => println!("{}", text_that_was_read.len()),
        Err(why_this_failed) => panic!("Failed to read file: {}", why_this_failed),
    }
}
