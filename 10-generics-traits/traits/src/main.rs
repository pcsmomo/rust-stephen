mod basket;
mod stack;

use basket::Basket;
use stack::Stack;

fn main() {
    let b1 = Basket::new(String::from("hi there"));
    let b2 = Basket::new(10);
    let b3 = Basket::new(true);

    let mut s1 = Stack::new(vec![String::from("hi there"), String::from("hi there 2")]);
    let mut s2 = Stack::new(vec![1, 2, 3]);

    s1.put(String::from("hi there 3"));
    s2.put(4);

    println!("s1: {:#?}", s1);
    println!("s2: {:#?}", s2);

    println!("b1: {:#?}", b1);
    println!("b2: {:#?}", b2);
    println!("b3: {:#?}", b3);
}
