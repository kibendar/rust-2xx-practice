fn page_count(n: i32, p: i32) -> i32 {
    // Calculate the number of flips from the front
    let front = p / 2;

    // Calculate the number of flips from the back
    let back = (n / 2) - (p / 2);

    // Return the minimum of the two
    std::cmp::min(front, back)
}

#[test]
fn main() {
    // Sample input
    let n = 5; // Total pages
    let p = 3; // Target page

    // Call the function and print the result
    println!("{}", page_count(n, p));  // Expected output: 1
}
