fn main() {
    // String
    let colour = String::from("Blue");
    println!("Colour: {}", colour);

    // &str
    let portion = &colour[1..4];
    println!("Portion: {}", portion);

    // &String
    let colour_ref = &colour;
    println!("Colour Ref: {}", colour_ref);

    // &str
    let name = "Noah";
    let name_slice = &name[0..4];
    println!("Name: {}", name_slice);

    // &str
    let red = String::from("Red");
    let r = red.as_str();
    println!("R: {}", r);
}
