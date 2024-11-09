use std::collections::HashMap;

fn sockMerchant(n: usize, ar: Vec<i32>) -> i32 {
    // Create a hashmap to store the frequency of each color
    let mut color_count = HashMap::new();

    // Count the occurrences of each color
    for color in ar {
        let counter = color_count.entry(color).or_insert(0);
        *counter += 1;
    }

    // Calculate the total number of pairs
    let mut total_pairs = 0;

    // For each color, calculate the number of pairs
    for &count in color_count.values() {
        total_pairs += count / 2;  // integer division gives number of pairs
    }

    total_pairs
}

#[test]
fn main() {
    // Sample input
    let n = 9;
    let ar = vec![10, 20, 20, 10, 10, 30, 50, 10, 20];

    // Call the function and print the result
    let result = sockMerchant(n, ar);
    println!("{}", result);  // Expected output: 3
}
