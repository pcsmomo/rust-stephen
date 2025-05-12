fn add_two_numbers(a: i32, b: i32) -> i32 {
    let result = a + b;

    if a > b { result } else { result + 1 }
}

fn main() {
    let sum = add_two_numbers(5, 7);
    println!("The sum of 5 and 7 is: {}", sum);
}
