fn main() {
    let number = 7;

    match number {
        1 => println!("One"),
        2 | 3 => println!("Two or Three"),
        4..=7 => println!("Four to Seven"),
        _ => println!("Something else"),
    }
}
