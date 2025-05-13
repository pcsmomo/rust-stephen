fn device(a: f64, b: f64) -> _ {
    if b == 0.0 {
        // error handling
    }

    a / b
}

fn main() {
    println!("{:?}", device(10.0, 0.0));
}
