fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];

    numbers.push(6);

    for number in numbers {
        println!("Number: {}", number);
    }

    // println!("Numbers: {:#?}", numbers);
}
