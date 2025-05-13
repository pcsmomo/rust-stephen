use std::fs;
use std::io::Error;

fn extract_error(text: &str) -> Vec<&str> {
    let split_text = text.split("\n");

    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line);
        }
    }

    results
}

// 3. Use propagated version
fn main() -> Result<(), Error> {
    // constants
    const FILE_NAME: &str = "logs.txt";

    // 3.
    let text = fs::read_to_string(FILE_NAME)?;
    let error_logs = extract_error(text.as_str());
    fs::write("errors.txt", error_logs.join("\n"))?;

    Ok(())
}

// // 2. Short version - using `expect`
// fn main() {
//     // constants
//     const FILE_NAME: &str = "logs.txt";

//     let text = fs::read_to_string(FILE_NAME).expect(&format!("Failed to read {}", FILE_NAME));
//     let error_logs = extract_error(text.as_str());
//     fs::write("errors.txt", error_logs.join("\n")).expect("Failed to write to file");
// }

// // 1. Long version - using match
// fn main() {
//     // constants
//     const FILE_NAME: &str = "logs.txt";

//     match fs::read_to_string(FILE_NAME) {
//         Ok(text_that_was_read) => {
//             // error_logs = extract_error_and_return(text_that_was_read.as_str());

//             let local_error_logs = extract_error(text_that_was_read.as_str());
//             println!("{:#?}", local_error_logs);

//             match fs::write("errors.txt", local_error_logs.join("\n")) {
//                 Ok(_) => println!("Wrote errors.txt"),
//                 Err(why_this_failed) => {
//                     panic!("Failed to write to file: {}", why_this_failed);
//                 }
//             }
//         }
//         Err(why_this_failed) => {
//             panic!("Failed to read file: {}", why_this_failed);
//         }
//     }
// }
