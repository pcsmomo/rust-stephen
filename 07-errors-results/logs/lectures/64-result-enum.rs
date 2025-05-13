use std::io::Error;

fn divide(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        Err(Error::other("can't divide by 0"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    match divide(5.0, 0.0) {
        Ok(result_of_division) => println!("result: {}", result_of_division),
        Err(what_went_wrong) => println!("error: {}", what_went_wrong),
    }
}
