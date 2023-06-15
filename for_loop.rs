fn main() {
    // Iterate over a range
    for number in 1..=5 {
        println!("Number: {}", number);
    }

    // Iterate over an array
    let array = [10, 20, 30, 40, 50];
    for element in &array {
        println!("Element: {}", element);
    }

    // Iterate over a vector
    let vector = vec!["apple", "banana", "cherry"];
    for fruit in vector {
        println!("Fruit: {}", fruit);
    }
}
