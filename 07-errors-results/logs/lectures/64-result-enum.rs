use std::io::Error;

fn device(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        // error handling
        Err(Error::other("cant divide by 0"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    println!("{:?}", device(10.0, 0.0));
}
