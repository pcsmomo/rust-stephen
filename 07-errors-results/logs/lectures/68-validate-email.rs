use std::io::Error;

fn validate_email(email: String) -> Result<(), Error> {
    // Using regex to properly validate email format
    // let email_regex =
    //     regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    // if email_regex.is_match(&email) {
    if email.contains("@") {
        // using empty tuple as it's a convention to return a value when the function is successful
        Ok(())
    } else {
        Err(Error::other("emails must have an @"))
    }
}

fn main() {
    let email = String::from("noah@gmail.com");

    match validate_email(email) {
        Ok(_) => println!("email is valid"),
        Err(reason_this_failed_validation) => panic!("error: {}", reason_this_failed_validation),
    }
}
