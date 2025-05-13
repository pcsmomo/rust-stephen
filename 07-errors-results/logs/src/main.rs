use std::fs;

// Downside:
// results leaves in heap memory and if the text size is huge, it will take a lot of memory

// fn extract_error_and_return(text: &str) -> Vec<String> {
//     let split_text = text.split("\n");

//     let mut results = vec![];

//     for line in split_text {
//         if line.starts_with("ERROR") {
//             results.push(line.to_string());
//         }
//     }

//     results
// }

fn extract_error_and_return(text: &str) -> Vec<&str> {
    let split_text = text.split("\n");

    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line);
        }
    }

    results
}

fn main() {
    // let mut error_logs = vec![];

    match fs::read_to_string("logs.txt") {
        Ok(text_that_was_read) => {
            // error_logs = extract_error_and_return(text_that_was_read.as_str());

            let local_error_logs = extract_error_and_return(text_that_was_read.as_str());
            println!("{:#?}", local_error_logs);

            match fs::write("errors.txt", local_error_logs.join("\n")) {
                Ok(_) => println!("Wrote errors.txt"),
                Err(why_this_failed) => {
                    panic!("Failed to write to file: {}", why_this_failed);
                }
            }
        }
        Err(why_this_failed) => {
            panic!("Failed to read file: {}", why_this_failed);
        }
    }

    // println!("{:#?}", error_logs);
}
